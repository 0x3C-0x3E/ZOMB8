use crate::client::Client;
use game::ecs::{
    entities::player::Player,
    network_id::NetworkId,
    systems::{input::input_system, physics::physics_system_for_entity},
    transform::{Position, Velocity},
};
use hecs::Entity;
use protocol::{TPS, packets::snapshot::PacketSnapshot};

pub fn snapshot_handler(client: &mut Client, packet_snapshot: PacketSnapshot) {
    for (id, new_state) in packet_snapshot.players {
        let found = client
            .state
            .world
            .query_mut::<(Entity, &NetworkId, &mut Position, &mut Velocity)>()
            .with::<&Player>()
            .into_iter()
            .find(|(_, net_id, _, _)| **net_id == id)
            .map(|(e, _, pos, vel)| {
                pos.update_vec2(new_state.pos);
                // vel.update_vec2(new_state.vel);
                (e, *pos, *vel)
            });
        if let Some((e, mut pos, mut vel)) = found {
            if id == client.client_id {
                client
                    .last_maps
                    .retain(|(seq, _)| *seq > packet_snapshot.last_ack_seq);

                for (_, map) in client.last_maps.iter() {
                    input_system(&mut client.state, client.client_id, map);
                    physics_system_for_entity(&mut pos, &mut vel, 1.0 / TPS as f32);
                }
            }
            let mut old_pos = client.state.world.get::<&mut Position>(e).unwrap();
            // let mut old_vel = self.state.world.get::<&mut Velocity>(e).unwrap();
            *old_pos = pos;
            // *old_vel = vel;
        } else {
            Player::spawn(&mut client.state.world, Position::from(new_state.pos), id);
        }
    }
}
