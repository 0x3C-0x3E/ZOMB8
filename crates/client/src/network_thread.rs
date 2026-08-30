use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};

use macroquad::prelude::*;
use protocol::{
    config_parser::{port, server_ip},
    packet::{MAX_DATAGRAM_SIZE, Packet},
    packets::ping::PacketPing,
};
use tokio::{
    net::UdpSocket,
    sync::{
        mpsc::{Receiver, Sender},
        watch,
    },
    time::Instant,
};

pub async fn client_network_loop(
    out_send: Sender<Packet>,
    mut in_recv: Receiver<Packet>,
    mut input_recv: watch::Receiver<Packet>,
) -> anyhow::Result<()> {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(addr).await?;

    let addr = SocketAddrV4::new(server_ip().parse()?, port());
    socket.connect(addr).await?;

    let mut last_sent = Instant::now();

    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];
    loop {
        let ping_deadline = last_sent + Duration::from_secs(2);
        tokio::select! {
            result = socket.recv(&mut buf) => {
                let len = result?;
                let recv_packet = Packet::try_from(&buf[..len]);
                if let Ok(recv_packet) = recv_packet {
                    let _ = out_send.send(recv_packet).await;
                }
            },
            Some(packet) = in_recv.recv() => {
                let buffer: Vec<u8> = (&packet).into();
                socket.send(&buffer).await?;
            },
            _ = input_recv.changed() => {
                let packet = input_recv.borrow_and_update().clone();
                let buffer: Vec<u8> = (&packet).into();
                socket.send(&buffer).await?;
            },
            _  = tokio::time::sleep_until(ping_deadline) => {
                let payload = PacketPing::new();
                if let Ok(packet) = Packet::from_payload(payload) {
                    let buffer: Vec<u8> = (&packet).into();
                    socket.send(&buffer).await?;
                    last_sent = Instant::now();
                }
            },

            else => break Ok(()),
        }
    }
}
