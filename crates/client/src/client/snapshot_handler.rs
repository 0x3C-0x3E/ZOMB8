use crate::{client::core::Client, client::spawn_despawn_handler::spawn_network_entity_from_state};
use game::ecs::{
    components::snapshot_sync::SnapshotSync,
    network_id::NetworkId,
    systems::{input::input_system_for_player, physics::physics_system_for_player},
    transform::{Position, Velocity},
};
use protocol::{TPS, packets::snapshot::PacketSnapshot};

pub const FIXED_DT: f32 = 1.0 / TPS as f32;

impl Client {
    pub fn snapshot_handler(&mut self, packet_snapshot: PacketSnapshot) {
        self.interp_timer = 0.0;
        for (id, new_state) in packet_snapshot.entities {
            let mut binding = self
                .state
                .world
                .query::<(&NetworkId, &mut Position, &mut Velocity)>()
                .with::<&SnapshotSync>();
            let found = binding
                .into_iter()
                .find(|(net_id, _, _)| **net_id == id)
                .map(|(_, pos, vel)| (pos, vel));

            if let Some((pos, vel)) = found {
                *pos = new_state.pos.into();
                *vel = new_state.vel.into();
                if id == self.client_id {
                    drop(binding);
                    self.last_maps
                        .retain(|(seq, _)| *seq > packet_snapshot.last_ack_seq);
                    if let Some(player) = self.player {
                        for (_, map) in self.last_maps.iter() {
                            {
                                let mut vel =
                                    self.state.world.get::<&mut Velocity>(player).unwrap();
                                input_system_for_player(&mut vel, map);
                            }
                            if let Some(player) = self.player {
                                physics_system_for_player(&self.state.world, player, FIXED_DT);
                            }
                        }
                    }
                }
            } else {
                drop(binding);
                spawn_network_entity_from_state(&mut self.state.world, new_state, id);
            }
        }
    }
}
