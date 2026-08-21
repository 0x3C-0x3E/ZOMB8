use macroquad::prelude::*;

use crate::{
    ecs::components::transform::{Position, Velocity},
    game::state::State,
};

pub fn physics_system(state: &mut State, dt: f32) {
    for (pos, vel) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
