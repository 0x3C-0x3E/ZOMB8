use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, Instant},
};

use game::{
    ecs::{entities::tile::Tile, transform::Position},
    game::state::State,
};
use protocol::packet::{MAX_DATAGRAM_SIZE, Packet};
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

    pub fn send_to_all(&mut self, packet: Packet) {
        for client in self.clients.keys() {}
    }

    pub fn insert_client(&mut self, sender_addr: SocketAddr) {
        self.clients.insert(sender_addr, Instant::now());
    }

    pub fn check_for_disconnects(&mut self) {
        self.clients
            .retain(|_, last_seen| last_seen.elapsed() < Duration::from_secs(60));
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut allocator = NetworkIdAllocator::new();

    let mut state = State::new();
    let _ = Tile::spawn(&mut state.world, Position::zero(), allocator.allocate());

    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];

    let mut server = Server::new().await?;

    loop {
        let (len, sender_addr) = server.socket.recv_from(&mut buf).await?;
        println!("{len} bytes recv from {sender_addr}");
        server.insert_client(sender_addr);

        server.check_for_disconnects();

        let packet = Packet::try_from(&buf[..]).unwrap(); // TODO: fix this
    }
}
