use hecs::World;

use crate::ecs::{
    components::transform::{Position, Velocity},
    entities::{player::Player, zombie::Zombie},
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

pub fn zombie_movement_system(world: &mut World) {
    for (pos, vel) in world
        .query::<(&Position, &mut Velocity)>()
        .with::<&Zombie>()
        .iter()
    {
        if let Some(closest) = get_closest_player_pos(world, pos) {
            let diff = (closest.vec2() - pos.vec2()).normalize_or_zero();

            vel.x = diff.x * 30.0;
            vel.y = diff.y * 20.0;
        }
    }
}
