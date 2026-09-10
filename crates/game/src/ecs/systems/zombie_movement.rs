use std::collections::{HashMap, VecDeque};

use hecs::{Entity, World};
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::{
    components::{health::Health, moveable::CollisionMesh, score::Score, transform::Position},
    entities::{
        bullet::{Bullet, LinkedPlayerId},
        player::Player,
        zombie::{Zombie, ZombieSpawnTimer},
    },
    network_id::NetworkId,
    systems::pathfinding::GridPos,
    transform::Velocity,
};

pub fn get_closest_entity<E>(world: &World, pos: &Position) -> Option<Entity>
where
    E: hecs::Query,
{
    let pos = pos.vec2();

    world
        .query::<(Entity, &Position)>()
        .with::<E>()
        .iter()
        .min_by_key(|(_, e_pos)| pos.distance_squared(e_pos.vec2()) as i64)
        .map(|(e, _)| e)
}

pub fn zombie_collision_system(world: &mut World) -> Vec<(Entity, NetworkId)> {
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

pub fn zombie_movement_system(
    world: &mut World,
    zombie_paths: &mut HashMap<Entity, VecDeque<GridPos>>,
    dt: f32,
) {
    for (e, pos, vel) in world
        .query::<(Entity, &mut Position, &mut Velocity)>()
        .with::<&Zombie>()
        .iter()
    {
        if world.get::<&ZombieSpawnTimer>(e).is_ok() {
            continue;
        }

        if let Some(path) = zombie_paths.get_mut(&e)
            && let Some(next_pos) = path.front()
        {
            let next_pos = Position::new(next_pos.x as f32 * 8.0, next_pos.y as f32 * 8.0);
            let to_target = next_pos.vec2() - pos.vec2();
            let dist = to_target.length();
            let dir = to_target.normalize_or_zero();

            let step = 30.0 * dt;

            if dist <= step {
                pos.x = next_pos.x;
                pos.y = next_pos.y;
                path.pop_front();
                vel.x = 0.0;
                vel.y = 0.0;
            } else {
                vel.x = dir.x * 30.0;
                vel.y = dir.y * 30.0;
            }

            let diff = to_target.normalize_or_zero();
            vel.x = diff.x * 30.0;
            vel.y = diff.y * 30.0;
        } else {
            vel.x = 0.0;
            vel.y = 0.0;
        }
    }
}
