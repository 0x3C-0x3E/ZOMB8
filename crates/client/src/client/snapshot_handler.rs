use crate::client::core::Client;
use game::ecs::{
    components::{health::Health, score::Score, snapshot_sync::SnapshotSync},
    entities::zombie::ZombieSpawnTimer,
    network_id::NetworkId,
    systems::{input::input_system_for_player, physics::physics_system_for_player},
    transform::{Position, Velocity},
};
use protocol::{config_parser::tps, packets::snapshot::PacketSnapshot};

impl Client {
    pub fn snapshot_handler(&mut self, packet_snapshot: PacketSnapshot) {
        let fixed_dt: f32 = 1.0 / tps() as f32;

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
                if id == self.player_id {
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
                                physics_system_for_player(&self.state.world, player, fixed_dt);
                            }
                        }
                    }
                }
            } else {
                drop(binding);
                self.spawn_network_entity_from_state(new_state, id);
            }
        }

        for (id, new_health) in packet_snapshot.entity_health {
            let Some(prev_health) = self
                .state
                .world
                .query_mut::<(&NetworkId, &mut Health)>()
                .with::<&SnapshotSync>()
                .into_iter()
                .find(|(net_id, _)| **net_id == id)
                .map(|(_, health)| health)
            else {
                continue;
            };

            prev_health.health = new_health.0;
            prev_health.max = new_health.1;
        }

        for (id, new_score) in packet_snapshot.player_scores {
            let Some(prev_score) = self
                .state
                .world
                .query_mut::<(&NetworkId, &mut Score)>()
                .with::<&SnapshotSync>()
                .into_iter()
                .find(|(net_id, _)| **net_id == id)
                .map(|(_, score)| score)
            else {
                continue;
            };

            prev_score.0 = new_score;
        }

        for (id, _kind, new_time) in packet_snapshot.timers {
            // TODO: use kind
            let Some(timer) = self
                .state
                .world
                .query_mut::<(&NetworkId, &mut ZombieSpawnTimer)>()
                .with::<&SnapshotSync>()
                .into_iter()
                .find(|(net_id, _)| **net_id == id)
                .map(|(_, timer)| timer)
            else {
                continue;
            };

            timer.0 = new_time;
        }
    }
}
