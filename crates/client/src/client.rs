use std::collections::{VecDeque, vec_deque};

use crate::{
    snapshot_handler::snapshot_handler,
    spawn_despawn_handler::{despawn_network_entity, spawn_network_entity},
};
use game::{
    ecs::{
        entities::player::Player,
        network_id::NetworkId,
        transform::{Position, RenderPosition},
    },
    game::state::State,
};
use hecs::Entity;
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

    pub player: Option<Entity>,
    pub prev_pos: Position,

    pub last_snapshots: VecDeque<PacketSnapshot>,
    pub interp_timer: f32,
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

            player: None,
            prev_pos: Position::zero(),

            last_snapshots: VecDeque::with_capacity(5),
            interp_timer: 0.0,
        }
    }

    pub fn set_client_id(&mut self, id: NetworkId) {
        self.client_id = id;
    }

    pub fn set_local_player_prev_pos(&mut self) {
        if let Some(player) = self.player {
            self.prev_pos = *self
                .state
                .world
                .get::<&Position>(player)
                .expect("player has no position");
        }
    }

    pub fn set_local_player_render_pos(&mut self, alpha: f32) {
        if let Some(player) = self.player
            && let Ok((render_pos, pos)) = self
                .state
                .world
                .query_one::<(&mut RenderPosition, &Position)>(player)
                .get()
        {
            render_pos.lerp(&self.prev_pos, pos, alpha);
        }
    }

    pub fn set_render_pos(&mut self, alpha: f32) {
        let prev_snapshot = self.last_snapshots.iter().rev().nth(1);
        for (render_pos, pos, id) in self
            .state
            .world
            .query_mut::<(&mut RenderPosition, &Position, &NetworkId)>()
            .with::<&Player>()
        {
            if *id == self.client_id {
                continue;
            }

            if let Some(prev_snapshot) = prev_snapshot
                && let Some(prev_pos) = prev_snapshot
                    .players
                    .iter()
                    .find(|(pid, _)| pid == id)
                    .map(|(_, state)| state.pos)
            {
                render_pos.lerp(&Position::new(prev_pos.x, prev_pos.y), pos, alpha);
            } else {
                render_pos.lerp(pos, pos, alpha);
            }
        }
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
                let player = self
                    .state
                    .world
                    .query_mut::<(Entity, &NetworkId)>()
                    .with::<&Player>()
                    .into_iter()
                    .find(|(_, n)| **n == self.client_id)
                    .map(|(e, _)| e)
                    .expect("client player does not exist");

                self.player = Some(player);
            }
            PacketKind::Snapshot => {
                let packet_snapshot: PacketSnapshot = bincode::deserialize(&packet.payload)?;
                self.last_snapshots.push_back(packet_snapshot.clone());
                if self.last_snapshots.len() > 5 {
                    self.last_snapshots.pop_front();
                }
                snapshot_handler(self, packet_snapshot);
            }
            _ => panic!("unhandled packet kind '{:?}'", packet.kind),
        }
        Ok(())
    }
}
