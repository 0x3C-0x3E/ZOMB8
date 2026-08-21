use game::{
    ecs::{
        entities::player::Player,
        network_id::NetworkId,
        systems::{input::input_system, physics::physics_system},
        transform::Position,
    },
    game::state::State,
};
use protocol::{
    TPS,
    network_id::ProtocolNetworkId,
    packet::Packet,
    packets::{
        input::{InputMap, PacketInput},
        set_player_id::PacketSetPlayerId,
        snapshot::PacketSnapshot,
        spawn_entity::PacketSpawnEntity,
    },
};
use tokio::sync::watch;

use crate::spawn_network_entity::spawn_network_entity;

pub struct Client {
    pub state: State,
    pub client_id: NetworkId,

    input_send: watch::Sender<Packet>,
    input_seq: u32,
    last_maps: Vec<(u32, InputMap)>,
}

impl Client {
    pub fn new(input_send: watch::Sender<Packet>) -> Self {
        Self {
            state: State::new(),
            client_id: ProtocolNetworkId(0),

            input_send,
            input_seq: 0,
            last_maps: vec![],
        }
    }

    pub fn set_client_id(&mut self, id: NetworkId) {
        self.client_id = id;
    }

    pub fn check_for_new_input(&mut self, current_map: &InputMap) -> anyhow::Result<()> {
        if self.last_maps.last().is_none() || *current_map != self.last_maps.last().unwrap().1 {
            self.input_seq += 1;
            let payload = PacketInput::new(self.client_id, self.input_seq, current_map.clone());
            let packet = Packet::from_payload(payload)?;
            self.input_send.send(packet)?;

            self.last_maps.push((self.input_seq, current_map.clone()));
        }

        Ok(())
    }

    pub fn handle_packet(&mut self, packet: Packet) {
        use protocol::packet::PacketKind;
        match packet.kind {
            PacketKind::SpawnEntity => {
                let packet_spawn_entity: PacketSpawnEntity =
                    bincode::deserialize(&packet.payload).unwrap();
                spawn_network_entity(&mut self.state.world, packet_spawn_entity);
            }
            PacketKind::SetPlayerId => {
                let packet_set_player_id: PacketSetPlayerId =
                    bincode::deserialize(&packet.payload).unwrap();
                println!("this client has id: {:?}", packet_set_player_id.id);
                self.set_client_id(packet_set_player_id.id);
            }
            PacketKind::Snapshot => {
                let packet_snapshot: PacketSnapshot =
                    bincode::deserialize(&packet.payload).unwrap();
                for (id, new_pos) in packet_snapshot.players {
                    let found = self
                        .state
                        .world
                        .query_mut::<(&NetworkId, &mut Position)>()
                        .with::<&Player>()
                        .into_iter()
                        .find(|(e_id, _)| **e_id == id)
                        .map(|(_, pos)| pos);
                    if let Some(pos) = found {
                        pos.update_vec2(new_pos);
                        if id == self.client_id {
                            self.last_maps
                                .retain(|(seq, _)| *seq > packet_snapshot.last_ack_seq);

                            for (_, map) in self.last_maps.iter() {
                                input_system(&mut self.state, self.client_id, map);
                                physics_system(&mut self.state, 1.0 / TPS as f32);
                            }
                        }
                    } else {
                        Player::spawn(&mut self.state.world, Position::from(new_pos), id);
                    }
                }
            }
            _ => panic!("unhandled packet kind '{:?}'", packet.kind),
        }
    }
}
