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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut allocator = NetworkIdAllocator::new();

    let mut state = State::new();
    let _ = Tile::spawn(&mut state.world, Position::zero(), allocator.allocate());

    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];
    let socket = UdpSocket::bind("[::1]:6969".parse::<SocketAddr>()?).await?;
    let mut clients: HashMap<SocketAddr, Instant> = HashMap::new();

    loop {
        let (len, sender_addr) = socket.recv_from(&mut buf).await?;
        println!("{len} bytes recv from {sender_addr}");

        clients.insert(sender_addr, Instant::now());
        clients.retain(|_, last_seen| last_seen.elapsed() < Duration::from_secs(60));
        let packet = Packet::try_from(&buf[..]).unwrap(); // TODO: fix this
    }
}
