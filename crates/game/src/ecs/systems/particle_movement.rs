use hecs::{Entity, World};
use macroquad::time::get_frame_time;

use crate::ecs::{
    entities::particle::Particle,
    transform::{Position, Velocity},
};

pub fn particle_movement_system(world: &mut World) {
    for (_e, pos, vel) in world
        .query::<(Entity, &mut Position, &Velocity)>()
        .with::<&Particle>()
        .iter()
    {
        // Note: this has to be done separately because the client does not call physics_system()
        pos.x += vel.x * get_frame_time();
        pos.y += vel.x * get_frame_time();
    }
}
