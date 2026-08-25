use glam::Vec2;
use hecs::World;

use crate::ecs::{
    components::{sprite::Rotation, transform::Velocity},
    entities::bullet::Bullet,
};

pub fn bullet_movement_system(world: &mut World) {
    for (vel, rotation) in world
        .query_mut::<(&mut Velocity, &Rotation)>()
        .with::<&Bullet>()
    {
        let mut vec_vel = Vec2::from_angle(rotation.0.to_radians());
        vec_vel *= 140.0;
        vel.update_vec2(vec_vel);
    }
}
