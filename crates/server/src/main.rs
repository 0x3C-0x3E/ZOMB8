use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use game::{
    ecs::{entities::tile::Tile, transform::Position},
    game::state::State,
};
use protocol::{
    packet::{MAX_DATAGRAM_SIZE, Packet},
    ping::PacketPing,
    spawn_entity::{EntityKind, PacketSpawnEntity},
};
use tokio::net::UdpSocket;

use crate::network_id_allocator::NetworkIdAllocator;

mod network_id_allocator;

pub struct Server {
    pub socket: UdpSocket,
    pub clients: HashMap<SocketAddr, Instant>,
}

impl Server {
    pub async fn new() -> anyhow::Result<Self> {
        let socket = UdpSocket::bind("[::1]:6969".parse::<SocketAddr>()?).await?;
        let clients: HashMap<SocketAddr, Instant> = HashMap::new();
        Ok(Self { socket, clients })
    }

    pub async fn send_to_all(&mut self, packet: Packet) {
        let buffer: Vec<u8> = (&packet).into();
        for client in self.clients.keys() {
            let _ = self.socket.send_to(&buffer, client).await;
        }
    }

    pub async fn send_to(
        &mut self,
        packet: &Packet,
        sender_addr: SocketAddr,
    ) -> anyhow::Result<()> {
        let buffer: Vec<u8> = packet.into();
        self.socket.send_to(&buffer, sender_addr).await?;
        Ok(())
    }

    pub fn check_insert_client(&mut self, sender_addr: SocketAddr) -> bool {
        self.clients.insert(sender_addr, Instant::now()).is_none()
    }

    pub fn check_for_disconnects(&mut self) {
        self.clients
            .retain(|_, last_seen| last_seen.elapsed() < Duration::from_secs(60));
    }
}

pub fn handle_packet(_state: &mut State, packet: Packet) {
    println!("kind: {:?}", packet);
    use protocol::packet::PacketKind;
    match packet.kind {
        PacketKind::SpawnEntity => {
            println!("recv spawn entity packet -> this cannot be sent to a server");
        }
        PacketKind::Ping => {
            let packet_ping: PacketPing = bincode::deserialize(&packet.payload).unwrap();
            println!("recv ping: {}", packet_ping.now);
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut allocator = NetworkIdAllocator::new();

    let mut state = State::new();

    let network_id = allocator.allocate();
    let _ = Tile::spawn(&mut state.world, Position::zero(), network_id);

    let payload = PacketSpawnEntity::new(network_id, EntityKind::Tile, Position::zero().into());
    let spawn_packet = Packet::from_payload(payload).unwrap();

    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];

    let mut server = Server::new().await?;

    loop {
        let (len, sender_addr) = server.socket.recv_from(&mut buf).await?;
        if server.check_insert_client(sender_addr) {
            // if is new client
            let _ = server.send_to(&spawn_packet, sender_addr).await;
        }

        server.check_for_disconnects();

        let recv_packet = Packet::try_from(&buf[..len]).unwrap();
        handle_packet(&mut state, recv_packet);
    }
}
