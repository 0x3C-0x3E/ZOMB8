use hecs::{Entity, World};
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::{
    components::{health::Health, moveable::CollisionMesh, score::Score, transform::Position},
    entities::{
        bullet::{Bullet, LinkedPlayerId},
        player::Player,
        zombie::Zombie,
    },
    network_id::NetworkId,
};

fn get_closest_entity<E>(world: &World, pos: &Position) -> Option<Entity>
where
    E: hecs::Query,
{
    let mut closest: Option<(Entity, Position)> = None;
    for (e, b_pos) in world.query::<(Entity, &Position)>().with::<E>().iter() {
        if let Some(prev_closest) = closest {
            if pos.vec2().distance_squared(prev_closest.1.vec2())
                > pos.vec2().distance_squared(b_pos.vec2())
            {
                closest = Some((e, *b_pos));
            }
        } else {
            if pos.vec2().distance_squared(b_pos.vec2()) <= 128.0 {
                closest = Some((e, *b_pos))
            }
        }
    }

    closest.map(|(e, _)| e)
}

pub fn zombie_movement_system(world: &mut World) -> Vec<(Entity, NetworkId)> {
    let mut zombies_to_remove = Vec::new();
    for (e, pos, mesh, health, id) in world
        .query::<(Entity, &Position, &CollisionMesh, &mut Health, &NetworkId)>()
        .with::<&Zombie>()
        .iter()
    {
        if mesh.any_of_kind(EntityKind::Bullet) {
            health.health = health.get().saturating_sub(25);
            if health.get() == 0 {
                let Some(bullet) = get_closest_entity::<&Bullet>(world, pos) else {
                    continue;
                };
                let p_id = world.get::<&LinkedPlayerId>(bullet).unwrap();
                let Some(player) = world
                    .query::<(Entity, &NetworkId)>()
                    .with::<&Player>()
                    .into_iter()
                    .find(|(_, n)| **n == p_id.0)
                    .map(|(e, _)| e)
                else {
                    continue;
                };

                let mut score = world.get::<&mut Score>(player).unwrap();
                score.0 += 100;
                zombies_to_remove.push((e, *id));
            }
        }

        if mesh.any_of_kind(EntityKind::Player) {
            let Some(player) = get_closest_entity::<&Player>(world, pos) else {
                continue;
            };
            let mut health = world.get::<&mut Health>(player).unwrap();
            health.health = health.health.saturating_sub(10);
        }
    }

    zombies_to_remove
}
