use crate::network_thread::server_network_loop;
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use hecs::Entity;

use game::{
    ecs::{
        entities::player::Player,
        network_id::NetworkId,
        systems::input::input_system,
        transform::{Position, Velocity},
    },
    game::state::State,
};
use protocol::{
    packet::Packet,
    packets::{
        despawn_entity::PacketDespawnEntity,
        input::PacketInput,
        ping::PacketPing,
        set_player_id::PacketSetPlayerId,
        snapshot::{PacketSnapshot, PlayerState},
        spawn_entity::{EntityKind, PacketSpawnEntity},
    },
};
use tokio::{sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use crate::network_id_allocator::NetworkIdAllocator;

pub struct Server {
    pub allocator: NetworkIdAllocator,
    pub state: State,

    pub tick: u64,

    pub network_thread: JoinHandle<anyhow::Result<()>>,
    pub clients: HashMap<SocketAddr, Instant>,

    pub out_recv: Receiver<(SocketAddr, Packet)>,
    pub in_send: Sender<(SocketAddr, Packet)>,

    pub client_ids: HashMap<SocketAddr, NetworkId>,
    pub client_input_seq: HashMap<NetworkId, u32>,
}

impl Server {
    pub async fn new() -> anyhow::Result<Self> {
        let clients: HashMap<SocketAddr, Instant> = HashMap::new();

        let (out_send, out_recv) = tokio::sync::mpsc::channel::<(SocketAddr, Packet)>(100);
        let (in_send, in_recv) = tokio::sync::mpsc::channel::<(SocketAddr, Packet)>(100);

        let network_thread = tokio::spawn(server_network_loop(out_send, in_recv));
        Ok(Self {
            allocator: NetworkIdAllocator::new(),
            state: State::new(),
            tick: 0,
            clients,
            network_thread,
            out_recv,
            in_send,
            client_ids: HashMap::new(),
            client_input_seq: HashMap::new(),
        })
    }

    pub async fn send_to_all(&mut self, packet: &Packet) {
        for client in self.clients.keys() {
            let _ = self.in_send.send((*client, packet.clone())).await;
        }
    }

    pub async fn send_to(
        &mut self,
        packet: &Packet,
        sender_addr: SocketAddr,
    ) -> anyhow::Result<()> {
        self.in_send.send((sender_addr, packet.clone())).await?;
        Ok(())
    }

    pub fn check_insert_client(&mut self, sender_addr: SocketAddr) -> bool {
        self.clients.insert(sender_addr, Instant::now()).is_none()
    }

    pub fn touch_client(&mut self, addr: SocketAddr) {
        self.clients.insert(addr, Instant::now());
    }

    pub async fn check_for_disconnects(&mut self) -> anyhow::Result<()> {
        let disconnected_clients: Vec<_> = self
            .clients
            .iter()
            .filter(|(_, last_seen)| last_seen.elapsed() > Duration::from_secs(5))
            .map(|(addr, _)| *addr)
            .collect();

        for addr in disconnected_clients {
            if let Some(id) = self.client_ids.remove(&addr) {
                println!("removed client {:?}", id.0);

                let payload = PacketDespawnEntity::new(id);
                let packet = Packet::from_payload(payload)?;
                let _ = self.send_to_all(&packet).await;

                self.client_input_seq.remove(&id);

                let found = self
                    .state
                    .world
                    .query::<(Entity, &NetworkId)>()
                    .into_iter()
                    .find(|(_, e_id)| **e_id == id)
                    .map(|(e, _)| e);

                if let Some(e) = found {
                    let _ = self.state.world.despawn(e);
                }
            }
            self.clients.remove(&addr);
        }

        Ok(())
    }

    pub async fn create_new_player(&mut self, sender_addr: SocketAddr) -> anyhow::Result<()> {
        let client_id = self.allocator.allocate();
        let client_player_pos = Position::new(0.0 + client_id.0 as f32 * 8.0, 20.0);
        let _ = Player::spawn(&mut self.state.world, client_player_pos, client_id);

        let payload =
            PacketSpawnEntity::new(client_id, EntityKind::Player, client_player_pos.into());
        let packet = Packet::from_payload(payload)?;
        let _ = self.send_to_all(&packet).await;

        let payload = PacketSetPlayerId::new(client_id);
        let packet = Packet::from_payload(payload)?;

        self.client_ids.insert(sender_addr, client_id);

        let _ = self.send_to(&packet, sender_addr).await;
        Ok(())
    }

    pub async fn send_snapshot(&mut self) {
        let players: Vec<(NetworkId, PlayerState)> = self
            .state
            .world
            .query_mut::<(&NetworkId, &Position, &Velocity)>()
            .with::<&Player>()
            .into_iter()
            .map(|(n, pos, vel)| (*n, PlayerState::new((*pos).into(), (*vel).into())))
            .collect();

        for client in self.clients.keys() {
            let id = self.client_ids.get(client);
            if let Some(id) = id {
                let last_ack_seq = self.client_input_seq.get(id).copied().unwrap_or(0);
                let payload = PacketSnapshot::new(self.tick, last_ack_seq, players.clone());
                let packet = Packet::from_payload(payload).unwrap();
                let _ = self.in_send.send((*client, packet)).await;
            }
        }
    }

    pub fn handle_packet(&mut self, packet: Packet) -> anyhow::Result<()> {
        use protocol::packet::PacketKind;
        match packet.kind {
            PacketKind::Ping => {
                let _packet_ping: PacketPing = bincode::deserialize(&packet.payload)?;
            }
            PacketKind::Input => {
                let packet_input: PacketInput = bincode::deserialize(&packet.payload)?;
                let found = self
                    .state
                    .world
                    .query_mut::<&NetworkId>()
                    .with::<&Player>()
                    .into_iter()
                    .any(|id| *id == packet_input.client_id);
                if found {
                    if let Some(prev_seq) = self.client_input_seq.get(&packet_input.client_id)
                        && *prev_seq > packet_input.seq
                    {
                        println!("got out of order packet -> ignoring");
                        return Ok(());
                    }
                    self.client_input_seq
                        .insert(packet_input.client_id, packet_input.seq);

                    input_system(
                        &mut self.state,
                        packet_input.client_id,
                        &packet_input.input_map,
                    );
                }
            }

            _ => {
                println!("unhandled packet kind '{:?}'!", packet.kind)
            }
        }
        Ok(())
    }
}
