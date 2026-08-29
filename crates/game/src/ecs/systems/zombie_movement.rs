use hecs::{Entity, World};
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::{
    components::{
        moveable::CollisionMesh,
        transform::{Position, Velocity},
    },
    entities::{
        player::Player,
        zombie::{self, Zombie},
    },
    network_id::NetworkId,
};

fn get_closest_player_pos(world: &World, pos: &Position) -> Option<Position> {
    let mut closest: Option<Position> = None;
    for player_pos in world.query::<&Position>().with::<&Player>().iter() {
        if let Some(prev_closest) = closest {
            if pos.vec2().distance_squared(prev_closest.vec2())
                > pos.vec2().distance_squared(player_pos.vec2())
            {
                closest = Some(*player_pos);
            }
        } else {
            closest = Some(*player_pos);
        }
    }

    closest
}

pub fn zombie_movement_system(world: &mut World) -> Vec<(Entity, NetworkId)> {
    let mut zombies_to_remove = Vec::new();
    for (e, pos, vel, mesh, id) in world
        .query::<(Entity, &Position, &mut Velocity, &CollisionMesh, &NetworkId)>()
        .with::<&Zombie>()
        .iter()
    {
        if mesh.any_of_kind(EntityKind::Bullet) {
            zombies_to_remove.push((e, *id));
        }

        if let Some(closest) = get_closest_player_pos(world, pos) {
            let diff = (closest.vec2() - pos.vec2()).normalize_or_zero();

            vel.x = diff.x * 30.0;
            vel.y = diff.y * 20.0;
        } else {
            vel.x = 0.0;
            vel.y = 0.0;
        }
    }

    zombies_to_remove
}
