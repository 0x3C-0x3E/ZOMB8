use crate::client::spawn_despawn_handler::despawn_network_entity;
use std::collections::VecDeque;

use game::{
    ecs::{
        components::snapshot_sync::SnapshotSync,
        entities::player::Player,
        network_id::NetworkId,
        systems::input::{get_input_map, input_system},
        transform::{Position, RenderPosition, Velocity},
    },
    game::state::State,
};
use glam::Vec2;
use hecs::Entity;
use macroquad::input::{MouseButton, is_key_down, is_key_pressed, is_mouse_button_pressed};
use protocol::{
    network_id::ProtocolNetworkId,
    packet::Packet,
    packets::{
        despawn_entity::PacketDespawnEntity,
        input::{InputMap, PacketInput},
        level_data::PacketLevelData,
        set_player_id::PacketSetPlayerId,
        snapshot::{EntityState, PacketSnapshot},
        spawn_entity::{EntityKind, PacketSpawnEntity},
    },
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    watch,
};

use crate::client::spawn_despawn_handler::{spawn_network_entity, spawn_network_entity_from_state};

pub struct Client {
    pub state: State,
    pub client_id: NetworkId,

    pub out_recv: Receiver<Packet>,
    pub in_send: Sender<Packet>,

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
    pub fn new(
        out_recv: Receiver<Packet>,
        in_send: Sender<Packet>,
        input_send: watch::Sender<Packet>,
    ) -> Self {
        Self {
            state: State::new(),
            client_id: ProtocolNetworkId(0),

            out_recv,
            in_send,

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

    pub fn get_player_state(&mut self) -> Option<(&mut Position, &mut Velocity)> {
        let player = self.player?;

        self.state
            .world
            .query_one_mut::<(&mut Position, &mut Velocity)>(player)
            .ok()
    }

    pub async fn send(&mut self, packet: Packet) {
        let _ = self.in_send.send(packet).await;
    }

    pub fn input_system(&mut self) {
        let input_map = get_input_map();
        let _ = self.check_for_new_input(&input_map);

        input_system(&mut self.state, self.client_id, &input_map);

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("shoot!");
        }
    }

    pub fn set_local_prev_pos(&mut self) {
        if let Some(player) = self.player {
            self.prev_pos = *self
                .state
                .world
                .get::<&Position>(player)
                .expect("player has no position");
        }
    }

    pub fn check_for_new_input(&mut self, current_map: &InputMap) -> anyhow::Result<()> {
        let changed = self.last_sent_map.as_ref() != Some(current_map);
        if changed {
            self.input_seq += 1;
            self.last_maps.push((self.input_seq, current_map.clone()));

            let payload = PacketInput::new(self.client_id, self.input_seq, current_map.clone());
            let packet = Packet::from_payload(payload)?;

            self.input_send.send_replace(packet);
            self.last_sent_map = Some(current_map.clone());
        }

        Ok(())
    }

    pub fn set_local_render_pos(&mut self, alpha: f32) {
        if let Some(player) = self.player
            && let Ok((render_pos, pos)) = self
                .state
                .world
                .query_one::<(&mut RenderPosition, &Position)>(player)
                .get()
        {
            render_pos.lerp(&self.prev_pos.vec2(), &pos.vec2(), alpha);
        }
    }

    pub fn set_net_render_pos(&mut self, alpha: f32) {
        let mut snapshot_iter = self.last_snapshots.iter().rev();
        let sn_after = snapshot_iter.next();
        let sn_before = snapshot_iter.next();

        for (render_pos, id, pos) in self
            .state
            .world
            .query_mut::<(&mut RenderPosition, &NetworkId, &Position)>()
            .with::<&SnapshotSync>()
        {
            if *id == self.client_id {
                continue;
            }
            let Some(sn_before) = sn_before else {
                render_pos.set(pos);
                continue;
            };

            let Some(sn_after) = sn_after else {
                render_pos.set(pos);
                continue;
            };

            let Some(pos_before) = sn_before
                .entities
                .iter()
                .find(|(eid, _)| eid == id)
                .map(|(_, state)| state.pos)
            else {
                render_pos.set(pos);
                continue;
            };

            let Some(pos_after) = sn_after
                .entities
                .iter()
                .find(|(eid, _)| eid == id)
                .map(|(_, state)| state.pos)
            else {
                render_pos.set(pos);
                continue;
            };

            render_pos.lerp(&pos_before, &pos_after, alpha);
        }
    }

    pub fn try_recv(&mut self) {
        while let Ok(packet) = self.out_recv.try_recv() {
            let _ = self.handle_packet(packet);
        }
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
                self.snapshot_handler(packet_snapshot);
            }
            PacketKind::LevelData => {
                let packet_level_data: PacketLevelData = bincode::deserialize(&packet.payload)?;
                for (id, pos) in packet_level_data.tiles {
                    spawn_network_entity_from_state(
                        &mut self.state.world,
                        EntityState {
                            kind: EntityKind::Tile,
                            pos,
                            vel: Vec2::ZERO,
                        },
                        id,
                    );
                }
            }
            _ => panic!("unhandled packet kind '{:?}'", packet.kind),
        }
        Ok(())
    }
}
