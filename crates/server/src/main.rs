use std::{
    collections::HashMap,
    net::SocketAddr,
    thread::sleep,
    time::{Duration, Instant},
};

use game::{
    ecs::{
        entities::{player::Player, tile::Tile},
        systems::physics::physics_system,
        transform::Position,
    },
    game::state::State,
};
use protocol::{
    packet::{MAX_DATAGRAM_SIZE, Packet},
    packets::{
        ping::PacketPing,
        set_player_id::PacketSetPlayerId,
        spawn_entity::{EntityKind, PacketSpawnEntity},
    },
};
use tokio::{net::UdpSocket, sync::mpsc::Receiver, sync::mpsc::Sender, task::JoinHandle};

use crate::network_id_allocator::NetworkIdAllocator;

mod network_id_allocator;

const TPS: u32 = 2;

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
}

pub fn handle_packet(_state: &mut State, packet: Packet) {
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
async fn server_network_loop(
    out_send: Sender<(SocketAddr, Packet)>,
    mut in_recv: Receiver<(SocketAddr, Packet)>,
) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("[::1]:6969".parse::<SocketAddr>()?).await?;
    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];
    loop {
        tokio::select! {
            result = socket.recv_from(&mut buf) => {
                let (len, sender_addr) = result?;
                if let Ok(recv_packet) = Packet::try_from(&buf[..len]) {
                    out_send.send((sender_addr, recv_packet)).await.unwrap();
                }
            },
            Some((client_addr, packet)) = in_recv.recv() => {
                let buffer: Vec<u8> = (&packet).into();
                socket.send_to(&buffer, client_addr).await?;
            },
            else => break Ok(()),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut server = Server::new().await?;

    loop {
        if server.network_thread.is_finished() {
            panic!(
                "network thread exited with {:?}",
                server.network_thread.await?
            );
        }
        while let Ok((sender_addr, packet)) = server.out_recv.try_recv() {
            if server.check_insert_client(sender_addr) {
                let _ = server.create_new_player(sender_addr).await;
            }

            server.check_for_disconnects();

            server.touch_client(sender_addr);
            handle_packet(&mut server.state, packet);
        }

        physics_system(&mut server.state, 1.0 / TPS as f32);

        sleep(Duration::from_millis((1000 / TPS).into()));
    }
}
