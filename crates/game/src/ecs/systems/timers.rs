use hecs::{Entity, World};

use crate::ecs::entities::zombie::ZombieSpawnTimer;

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
