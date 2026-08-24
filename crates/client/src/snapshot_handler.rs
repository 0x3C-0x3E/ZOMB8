use crate::{client::Client, spawn_despawn_handler::spawn_network_entity_from_state};
use game::ecs::{
    components::snapshot_sync::SnapshotSync,
    network_id::NetworkId,
    systems::{input::input_system_for_player, physics::physics_system_for_entity},
    transform::{Position, Velocity},
};
use hecs::Entity;
use protocol::{TPS, packets::snapshot::PacketSnapshot};

pub const FIXED_DT: f32 = 1.0 / TPS as f32;

pub fn snapshot_handler(client: &mut Client, packet_snapshot: PacketSnapshot) {
    client.interp_timer = 0.0;
    for (id, new_state) in packet_snapshot.entities {
        let found = client
            .state
            .world
            .query_mut::<(Entity, &NetworkId, &mut Position, &mut Velocity)>()
            .with::<&SnapshotSync>()
            .into_iter()
            .find(|(_, net_id, _, _)| **net_id == id)
            .map(|(e, _, pos, vel)| {
                pos.update_vec2(new_state.pos);
                (e, *pos, *vel)
            });
        if let Some((e, mut pos, mut vel)) = found {
            if id == client.client_id {
                client
                    .last_maps
                    .retain(|(seq, _)| *seq > packet_snapshot.last_ack_seq);

                for (_, map) in client.last_maps.iter() {
                    input_system_for_player(&mut vel, map);
                    physics_system_for_entity(&mut pos, &mut vel, FIXED_DT);
                }
            }
            let mut old_pos = client.state.world.get::<&mut Position>(e).unwrap();
            *old_pos = pos;
        } else {
            spawn_network_entity_from_state(&mut client.state.world, new_state, id);
        }
    }
}
