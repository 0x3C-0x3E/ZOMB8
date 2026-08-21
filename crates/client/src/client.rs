use crate::{
    snapshot_handler::snapshot_handler,
    spawn_despawn_handler::{despawn_network_entity, spawn_network_entity},
};
use game::{ecs::network_id::NetworkId, game::state::State};
use protocol::{
    network_id::ProtocolNetworkId,
    packet::Packet,
    packets::{
        despawn_entity::PacketDespawnEntity,
        input::{InputMap, PacketInput},
        set_player_id::PacketSetPlayerId,
        snapshot::PacketSnapshot,
        spawn_entity::PacketSpawnEntity,
    },
};
use tokio::sync::watch;

pub struct Client {
    pub state: State,
    pub client_id: NetworkId,

    pub input_send: watch::Sender<Packet>,
    pub input_seq: u32,
    pub last_maps: Vec<(u32, InputMap)>,
    pub last_sent_map: Option<InputMap>,
}

impl Client {
    pub fn new(input_send: watch::Sender<Packet>) -> Self {
        Self {
            state: State::new(),
            client_id: ProtocolNetworkId(0),

            input_send,
            input_seq: 0,
            last_maps: vec![],
            last_sent_map: None,
        }
    }

    pub fn set_client_id(&mut self, id: NetworkId) {
        self.client_id = id;
    }

    pub fn check_for_new_input(&mut self, current_map: &InputMap) -> anyhow::Result<()> {
        let changed = self.last_sent_map.as_ref() != Some(current_map);
        if changed {
            self.input_seq += 1;
            self.last_maps.push((self.input_seq, current_map.clone()));
            let payload = PacketInput::new(self.client_id, self.input_seq, current_map.clone());
            let packet = Packet::from_payload(payload)?;
            self.input_send.send(packet)?;
            self.last_sent_map = Some(current_map.clone());
        }

        Ok(())
    }

    pub fn handle_packet(&mut self, packet: Packet) -> anyhow::Result<()> {
        use protocol::packet::PacketKind;
        match packet.kind {
            PacketKind::SpawnEntity => {
                let packet_spawn_entity: PacketSpawnEntity = bincode::deserialize(&packet.payload)?;
                spawn_network_entity(&mut self.state.world, packet_spawn_entity);
            }
            PacketKind::DespawnEntity => {
                let packet_despawn_entity: PacketDespawnEntity =
                    bincode::deserialize(&packet.payload)?;
                despawn_network_entity(&mut self.state.world, packet_despawn_entity);
            }
            PacketKind::SetPlayerId => {
                let packet_set_player_id: PacketSetPlayerId =
                    bincode::deserialize(&packet.payload)?;
                println!("this client has id: {:?}", packet_set_player_id.id);
                self.set_client_id(packet_set_player_id.id);
            }
            PacketKind::Snapshot => {
                let packet_snapshot: PacketSnapshot = bincode::deserialize(&packet.payload)?;
                snapshot_handler(self, packet_snapshot);
            }
            _ => panic!("unhandled packet kind '{:?}'", packet.kind),
        }
        Ok(())
    }
}
