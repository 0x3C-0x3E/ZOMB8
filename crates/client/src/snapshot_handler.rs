use crate::{client::Client, spawn_despawn_handler::spawn_network_entity_from_state};
use game::ecs::{
    components::snapshot_sync::SnapshotSync,
    network_id::NetworkId,
    systems::{input::input_system_for_player, physics::physics_system_for_entity},
    transform::{Position, Velocity},
};
use protocol::{TPS, packets::snapshot::PacketSnapshot};

pub const FIXED_DT: f32 = 1.0 / TPS as f32;

pub fn snapshot_handler(client: &mut Client, packet_snapshot: PacketSnapshot) {
    client.interp_timer = 0.0;
    for (id, new_state) in packet_snapshot.entities {
        let found = client
            .state
            .world
            .query_mut::<(&NetworkId, &mut Position, &mut Velocity)>()
            .with::<&SnapshotSync>()
            .into_iter()
            .find(|(net_id, _, _)| **net_id == id)
            .map(|(_, pos, vel)| (pos, vel));

        if let Some((pos, vel)) = found {
            if id == client.client_id {
                client
                    .last_maps
                    .retain(|(seq, _)| *seq > packet_snapshot.last_ack_seq);

                for (_, map) in client.last_maps.iter() {
                    input_system_for_player(vel, map);
                    physics_system_for_entity(pos, vel, FIXED_DT);
                }
            }
            *pos = new_state.pos.into();
            *vel = new_state.vel.into();
        } else {
            spawn_network_entity_from_state(&mut client.state.world, new_state, id);
        }
    }
}
