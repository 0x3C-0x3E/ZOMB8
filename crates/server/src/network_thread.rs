use std::net::SocketAddr;

use protocol::packet::{MAX_DATAGRAM_SIZE, Packet};
use tokio::{net::UdpSocket, sync::mpsc::Receiver, sync::mpsc::Sender};

pub async fn server_network_loop(
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
