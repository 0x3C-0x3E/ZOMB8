use hecs::{Entity, World};
use protocol::packets::spawn_entity::EntityKind;

use crate::ecs::{
    components::{
        health::Health,
        moveable::CollisionMesh,
        score::Score,
        transform::{Position, Velocity},
    },
    entities::{
        bullet::{Bullet, LinkedPlayerId},
        player::Player,
        zombie::Zombie,
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
            closest = Some((e, *b_pos))
        }
    }

    closest.map(|(e, _)| e)
}

pub fn zombie_movement_system(world: &mut World) -> Vec<(Entity, NetworkId)> {
    let mut zombies_to_remove = Vec::new();
    for (e, pos, vel, mesh, health, id) in world
        .query::<(
            Entity,
            &Position,
            &mut Velocity,
            &CollisionMesh,
            &mut Health,
            &NetworkId,
        )>()
        .with::<&Zombie>()
        .iter()
    {
        if mesh.any_of_kind(EntityKind::Bullet) {
            health.health = health.get().saturating_sub(25);
            if health.get() == 0 {
                let bullet = get_closest_entity::<&Bullet>(world, pos).unwrap();
                let p_id = world.get::<&LinkedPlayerId>(bullet).unwrap();
                let player = world
                    .query::<(Entity, &NetworkId)>()
                    .with::<&Player>()
                    .into_iter()
                    .find(|(_, n)| **n == p_id.0)
                    .map(|(e, _)| e)
                    .expect("client player does not exist");
                let mut score = world.get::<&mut Score>(player).unwrap();
                score.0 += 100;
                zombies_to_remove.push((e, *id));
            }
        }

        if mesh.any_of_kind(EntityKind::Player) {
            let player = get_closest_entity::<&Player>(world, pos).unwrap();
            let mut health = world.get::<&mut Health>(player).unwrap();
            health.health = health.health.saturating_sub(10);
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
