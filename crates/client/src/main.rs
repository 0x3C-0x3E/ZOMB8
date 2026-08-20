#![allow(clippy::new_without_default)]

use std::{
    net::{Ipv6Addr, SocketAddrV6},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use game::{
    ecs::systems::rendering::rendering_system,
    game::{state::State, texture_manager::TextureManager},
};
use macroquad::prelude::*;
use protocol::{
    packet::{MAX_DATAGRAM_SIZE, Packet},
    ping::PacketPing,
    spawn_entity::PacketSpawnEntity,
};
use tokio::net::UdpSocket;

use crate::spawn_network_entity::spawn_network_entity;

mod spawn_network_entity;

fn window_conf() -> Conf {
    Conf {
        window_title: "".to_owned(),
        window_width: 600.0 as i32,
        window_height: 600.0 as i32,
        window_resizable: true,
        sample_count: 1,
        ..Default::default()
    }
}

fn handle_packet(state: &mut State, packet: Packet) {
    println!("kind: {:?}", packet);
    use protocol::packet::PacketKind;
    match packet.kind {
        PacketKind::SpawnEntity => {
            let packet_spawn_entity: PacketSpawnEntity =
                bincode::deserialize(&packet.payload).unwrap();
            spawn_network_entity(&mut state.world, packet_spawn_entity);
        }
        _ => panic!("unhandled packet kind '{:?}'", packet.kind),
    }
}

#[tokio::main]
async fn client_network_loop(
    out_send: Sender<Packet>,
    in_recv: Receiver<Packet>,
) -> anyhow::Result<()> {
    let addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);
    let socket = UdpSocket::bind(addr).await?;
    socket.connect("[::1]:6969").await?;

    let payload = PacketPing::new();
    let packet = Packet::from_payload(payload).unwrap();

    let buffer: Vec<u8> = (&packet).into();
    let _ = socket.send(&buffer).await.unwrap();

    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];
    loop {
        let len = socket.recv(&mut buf).await?;
        let recv_packet = Packet::try_from(&buf[..len]).unwrap();
        out_send.send(recv_packet).unwrap();
    }
}

#[macroquad::main(window_conf)]
async fn main() -> anyhow::Result<()> {
    set_default_filter_mode(FilterMode::Nearest);
    let mut state = State::new();

    let mut texture_manager = TextureManager::new();

    texture_manager
        .load_texture("assets/img/tileset.png", "tileset")
        .await;

    let (out_send, out_recv) = mpsc::channel::<Packet>();
    let (in_send, in_recv) = mpsc::channel::<Packet>();

    let _network_thread = thread::spawn(move || client_network_loop(out_send, in_recv));

    loop {
        if let Ok(packet) = out_recv.try_recv() {
            handle_packet(&mut state, packet);
        }

        rendering_system(&mut state, &texture_manager);
        next_frame().await;
    }
}
