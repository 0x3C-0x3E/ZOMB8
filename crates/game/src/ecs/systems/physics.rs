use macroquad::prelude::*;

use crate::{
    ecs::components::transform::{Position, Velocity},
    game::state::State,
};

pub fn physics_system_for_entity(pos: &mut Position, vel: &mut Velocity, dt: f32) {
    pos.x += vel.x * dt;
    pos.y += vel.y * dt;
}

pub fn physics_system(state: &mut State, dt: f32) {
    for (pos, vel) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        physics_system_for_entity(pos, vel, dt);
    }
}
