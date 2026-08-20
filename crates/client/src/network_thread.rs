use std::net::{Ipv6Addr, SocketAddrV6};

use macroquad::prelude::*;
use protocol::packet::{MAX_DATAGRAM_SIZE, Packet};
use tokio::{
    net::UdpSocket,
    sync::mpsc::{Receiver, Sender},
};

pub async fn client_network_loop(
    out_send: Sender<Packet>,
    mut in_recv: Receiver<Packet>,
) -> anyhow::Result<()> {
    let addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0);
    let socket = UdpSocket::bind(addr).await?;
    socket.connect("[::1]:6969").await?;

    let mut buf = vec![0u8; MAX_DATAGRAM_SIZE];
    loop {
        tokio::select! {
            result = socket.recv(&mut buf) => {
                let len = result?;
                let recv_packet = Packet::try_from(&buf[..len]).unwrap();
                out_send.send(recv_packet).await.unwrap();
            },

            Some(packet) = in_recv.recv() => {
                let buffer: Vec<u8> = (&packet).into();
                socket.send(&buffer).await?;
            }

            else => break Ok(()),
        }
    }
}
