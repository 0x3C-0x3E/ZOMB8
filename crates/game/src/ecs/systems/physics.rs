use hecs::{Entity, World};
use macroquad::prelude::*;

use crate::ecs::{
    components::transform::{Position, Velocity},
    systems::collision::{Axis, resolve_collision_system},
};

pub fn physics_system_for_player(world: &World, player: Entity, dt: f32) {
    {
        let vel = world.get::<&Velocity>(player).unwrap();
        let mut pos = world.get::<&mut Position>(player).unwrap();
        pos.x += vel.x * dt;
    }
    resolve_collision_system(world, Axis::X);
    {
        let vel = world.get::<&Velocity>(player).unwrap();
        let mut pos = world.get::<&mut Position>(player).unwrap();
        pos.y += vel.y * dt;
    }
    resolve_collision_system(world, Axis::Y);
}

pub fn physics_system(world: &World, dt: f32) {
    for (pos, vel) in world.query::<(&mut Position, &mut Velocity)>().iter() {
        pos.x += vel.x * dt;
    }
    resolve_collision_system(world, Axis::X);

    for (pos, vel) in world.query::<(&mut Position, &mut Velocity)>().iter() {
        pos.y += vel.y * dt;
    }
    resolve_collision_system(world, Axis::Y);
}
