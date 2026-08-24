use crate::{client::Client, spawn_despawn_handler::spawn_network_entity_from_state};
use game::ecs::{
    components::snapshot_sync::SnapshotSync,
    network_id::NetworkId,
    systems::{input::input_system_for_player, physics::physics_system_for_player},
    transform::{Position, Velocity},
};
use protocol::{TPS, packets::snapshot::PacketSnapshot};

pub const FIXED_DT: f32 = 1.0 / TPS as f32;

pub fn snapshot_handler(client: &mut Client, packet_snapshot: PacketSnapshot) {
    client.interp_timer = 0.0;
    for (id, new_state) in packet_snapshot.entities {
        let mut binding = client
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
            if id == client.client_id {
                drop(binding);
                client
                    .last_maps
                    .retain(|(seq, _)| *seq > packet_snapshot.last_ack_seq);
                if let Some(player) = client.player {
                    for (_, map) in client.last_maps.iter() {
                        {
                            let mut vel = client.state.world.get::<&mut Velocity>(player).unwrap();
                            input_system_for_player(&mut vel, map);
                        }
                        if let Some(player) = client.player {
                            physics_system_for_player(&client.state.world, player, FIXED_DT);
                        }
                    }
                }
            }
        } else {
            drop(binding);
            spawn_network_entity_from_state(&mut client.state.world, new_state, id);
        }
    }
}
