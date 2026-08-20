use crate::network_thread::server_network_loop;
use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use game::{
    ecs::{entities::player::Player, transform::Position},
    game::state::State,
};
use protocol::{
    packet::Packet,
    packets::{
        ping::PacketPing,
        set_player_id::PacketSetPlayerId,
        spawn_entity::{EntityKind, PacketSpawnEntity},
    },
};
use tokio::{sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use crate::network_id_allocator::NetworkIdAllocator;

pub const TPS: u32 = 2;

pub struct Server {
    pub allocator: NetworkIdAllocator,
    pub state: State,

    pub network_thread: JoinHandle<anyhow::Result<()>>,
    pub clients: HashMap<SocketAddr, Instant>,

    pub out_recv: Receiver<(SocketAddr, Packet)>,
    pub in_send: Sender<(SocketAddr, Packet)>,
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
            clients,
            network_thread,
            out_recv,
            in_send,
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

    pub fn check_for_disconnects(&mut self) {
        self.clients
            .retain(|_, last_seen| last_seen.elapsed() < Duration::from_secs(60));
    }

    pub async fn create_new_player(&mut self, sender_addr: SocketAddr) -> anyhow::Result<()> {
        let client_id = self.allocator.allocate();
        let client_player_pos = Position::new(0.0 + client_id.0 as f32 * 8.0, 20.0);
        let _ = Player::spawn(&mut self.state.world, client_player_pos, client_id);

        let payload =
            PacketSpawnEntity::new(client_id, EntityKind::Player, client_player_pos.into());
        let packet = Packet::from_payload(payload).unwrap();

        let _ = self.send_to_all(&packet).await;

        let payload = PacketSetPlayerId::new(client_id);
        let packet = Packet::from_payload(payload).unwrap();

        let _ = self.send_to(&packet, sender_addr).await;
        Ok(())
    }

    pub fn handle_packet(&mut self, packet: Packet) {
        println!("kind: {:?}", packet);
        use protocol::packet::PacketKind;
        match packet.kind {
            PacketKind::Ping => {
                let _packet_ping: PacketPing = bincode::deserialize(&packet.payload).unwrap();
            }

            _ => {
                println!("unhandled packet kind '{:?}'!", packet.kind)
            }
        }
    }
}
