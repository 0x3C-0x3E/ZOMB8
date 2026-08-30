use crate::{network_thread::server_network_loop, wave::WaveInfo};
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    net::SocketAddr,
    time::{Duration, Instant},
};

use glam::Vec2;
use hecs::{Entity, World};

use game::{
    ecs::{
        components::{health::Health, score::Score, snapshot_sync::SnapshotSync},
        entities::{bullet::Bullet, player::Player, tile::Tile, zombie::Zombie},
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
        level_data::PacketLevelData,
        ping::PacketPing,
        request::PacketRequest,
        set_player_id::PacketSetPlayerId,
        shoot::PacketShoot,
        snapshot::{EntityState, PacketSnapshot},
        spawn_entity::{
            EntityKind::{self},
            PacketSpawnEntity,
        },
    },
};
use tokio::{sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use crate::network_id_allocator::NetworkIdAllocator;

pub struct Server {
    pub allocator: NetworkIdAllocator,
    pub state: State,

    pub wave_info: WaveInfo,

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

        let mut allocator = NetworkIdAllocator::new();
        let mut state = State::new();
        Server::deserialize_world(&mut state.world, &mut allocator);

        Ok(Self {
            allocator,
            state,
            wave_info: WaveInfo::new(),
            tick: 0,
            clients,
            network_thread,
            out_recv,
            in_send,
            client_ids: HashMap::new(),
            client_input_seq: HashMap::new(),
        })
    }

    pub fn deserialize_world(world: &mut World, allocator: &mut NetworkIdAllocator) {
        let file = File::open("assets/world.bin")
            .expect("could not open world.bin -> maybe you are in the wrong dir");

        let mut reader = BufReader::new(file);

        let tiles: Vec<Position> =
            bincode::deserialize_from(&mut reader).expect("deserialization error on world.bin");

        for pos in tiles {
            let _ = Tile::spawn(world, pos, allocator.allocate());
        }
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
                println!("client {:?} got disconnected", id.0);

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
        let client_player_pos = Position::new(8.0, 20.0);
        let _ = Player::spawn(&mut self.state.world, client_player_pos, client_id);

        let payload =
            PacketSpawnEntity::new(client_id, EntityKind::Player, client_player_pos.into());
        let packet = Packet::from_payload(payload)?;
        let _ = self.send_to_all(&packet).await;

        self.client_ids.insert(sender_addr, client_id);

        Ok(())
    }

    pub async fn spawn_zombie(&mut self, pos: Position, max_health: u32) -> anyhow::Result<()> {
        let id = self.allocator.allocate();

        let _ = Zombie::spawn(&mut self.state.world, pos, id, Some(max_health));

        let payload = PacketSpawnEntity::new(id, EntityKind::Zombie, pos.into());
        let packet = Packet::from_payload(payload)?;
        let _ = self.send_to_all(&packet).await;

        Ok(())
    }

    pub async fn spawn_bullet(&mut self, packet_shoot: PacketShoot) -> anyhow::Result<()> {
        let id = self.allocator.allocate();

        let pos: Position = packet_shoot.pos.into();

        let _ = Bullet::spawn(
            &mut self.state.world,
            pos,
            Some(packet_shoot.rotation),
            id,
            packet_shoot.linked_player_id,
        );

        let payload = PacketSpawnEntity::new(id, EntityKind::Bullet, pos.into());
        let packet = Packet::from_payload(payload)?;
        let _ = self.send_to_all(&packet).await;

        Ok(())
    }

    pub async fn despawn_entity(&mut self, entity: Entity, id: NetworkId) -> anyhow::Result<()> {
        let payload = PacketDespawnEntity::new(id);
        let packet = Packet::from_payload(payload)?;
        let _ = self.send_to_all(&packet).await;

        self.state.world.despawn(entity)?;
        Ok(())
    }

    pub async fn send_snapshot(&mut self) {
        let entities: Vec<(NetworkId, EntityState)> = self
            .state
            .world
            .query_mut::<(&NetworkId, &Position, &Velocity, &SnapshotSync)>()
            .with::<&SnapshotSync>()
            .into_iter()
            .map(|(n, pos, vel, kind)| (*n, EntityState::new(kind.0, (*pos).into(), (*vel).into())))
            .collect();

        let entity_health: Vec<(NetworkId, (u32, u32))> = self
            .state
            .world
            .query_mut::<(&NetworkId, &Health)>()
            .into_iter()
            .map(|(id, health)| (*id, (health.get(), health.get_max())))
            .collect();

        let player_scores: Vec<(NetworkId, u32)> = self
            .state
            .world
            .query_mut::<(&NetworkId, &Score)>()
            .into_iter()
            .map(|(id, score)| (*id, score.get()))
            .collect();

        for client in self.clients.keys() {
            let id = self.client_ids.get(client);
            if let Some(id) = id {
                let last_ack_seq = self.client_input_seq.get(id).copied().unwrap_or(0);
                let payload = PacketSnapshot::new(
                    self.tick,
                    last_ack_seq,
                    entities.clone(),
                    entity_health.clone(),
                    player_scores.clone(),
                );
                let packet = Packet::from_payload(payload).unwrap();
                let _ = self.in_send.send((*client, packet)).await;
            }
        }
    }

    pub async fn send_level_data(&mut self, sender_addr: &SocketAddr) -> anyhow::Result<()> {
        let tiles: Vec<(NetworkId, Vec2)> = self
            .state
            .world
            .query_mut::<(&NetworkId, &Position)>()
            .with::<&Tile>()
            .into_iter()
            .map(|(n, pos)| (*n, pos.vec2()))
            .collect();

        let payload = PacketLevelData::new(tiles);
        let packet = Packet::from_payload(payload)?;

        self.send_to(&packet, *sender_addr).await
    }

    pub async fn handle_packet(
        &mut self,
        sender_addr: SocketAddr,
        packet: Packet,
    ) -> anyhow::Result<()> {
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
            PacketKind::Request => {
                use protocol::packets::request::RequestKind;
                let packet_request: PacketRequest = bincode::deserialize(&packet.payload)?;
                match packet_request.kind {
                    RequestKind::PlayerId => {
                        let Some(client_id) = self.client_ids.get(&sender_addr) else {
                            todo!(
                                "this client does not have a player but is somehow talking to us"
                            );
                        };

                        let payload = PacketSetPlayerId::new(*client_id);
                        let packet = Packet::from_payload(payload)?;

                        let _ = self.send_to(&packet, sender_addr).await;
                    }
                    RequestKind::LevelData => {
                        let _ = self.send_level_data(&sender_addr).await;
                    }
                    RequestKind::Ping => {
                        let payload = PacketPing::new();
                        let packet = Packet::from_payload(payload)?;

                        let _ = self.send_to(&packet, sender_addr).await;
                    }
                }
            }
            PacketKind::Shoot => {
                let packet_shoot: PacketShoot = bincode::deserialize(&packet.payload)?;
                let _ = self.spawn_bullet(packet_shoot).await;
            }

            _ => {
                println!("unhandled packet kind '{:?}'", packet.kind)
            }
        }
        Ok(())
    }
}
