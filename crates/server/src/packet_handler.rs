use crate::server::Server;
use std::net::SocketAddr;

use game::ecs::{entities::player::Player, network_id::NetworkId, systems::input::input_system};
use protocol::{
    packet::Packet,
    packets::{
        input::PacketInput, ping::PacketPing, request::PacketRequest,
        set_player_id::PacketSetPlayerId, shoot::PacketShoot, wave::PacketWave,
    },
};

impl Server {
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
                    RequestKind::Wave => {
                        let payload = PacketWave::new(self.state.current_wave);
                        let packet = Packet::from_payload(payload)?;

                        let _ = self.send_to_all(&packet).await;
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
