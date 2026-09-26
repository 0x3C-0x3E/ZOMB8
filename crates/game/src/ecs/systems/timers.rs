use hecs::{Entity, World};

use crate::ecs::entities::{player::PlayerShootTimer, zombie::ZombieSpawnTimer};

pub fn zombie_update_timers(world: &mut World, dt: f32) {
    let timers_to_remove: Vec<Entity> = world
        .query_mut::<(Entity, &mut ZombieSpawnTimer)>()
        .into_iter()
        .map(|(e, t)| {
            t.0 -= dt;
            (e, t)
        })
        .filter(|(_, t)| t.0 <= 0.0)
        .map(|(e, _)| e)
        .collect();

    for e in timers_to_remove {
        let _ = world.remove_one::<ZombieSpawnTimer>(e);
    }
}

pub fn players_update_shoot_timers(world: &mut World, dt: f32) {
    for t in world.query_mut::<&mut PlayerShootTimer>() {
        t.0 -= dt;
        if t.0 < 0.0 {
            t.0 = 0.0;
        }
    }
}
