use hecs::{Entity, World};
use macroquad::time::get_frame_time;

use crate::ecs::{
    entities::particle::{Particle, TimeAlive},
    transform::{Position, Velocity},
};

pub fn particle_movement_system(world: &mut World) {
    let mut particles_to_remove = Vec::new();
    for (e, alive, pos, vel) in world
        .query::<(Entity, &mut TimeAlive, &mut Position, &Velocity)>()
        .with::<&Particle>()
        .iter()
    {
        // Note: this has to be done separately because the client does not call physics_system()
        pos.x += vel.x * get_frame_time();
        pos.y += vel.y * get_frame_time();

        alive.0 += get_frame_time();
        if alive.0 > 1.0 {
            particles_to_remove.push(e);
        }
    }

    for e in particles_to_remove {
        let _ = world.despawn(e);
    }
}
