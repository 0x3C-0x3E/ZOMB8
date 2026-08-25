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
        let vec_vel = Vec2::from_angle(rotation.0.to_radians());
        vel.update_vec2(vec_vel);
    }
}
