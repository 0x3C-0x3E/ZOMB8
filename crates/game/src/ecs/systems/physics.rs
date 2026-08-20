use macroquad::prelude::*;

use crate::{
    ecs::components::transform::{Position, Velocity},
    game::state::State,
};

pub fn physics_system(state: &mut State) {
    let dt = get_frame_time().min(1.0 / 30.0);

    for (pos, vel) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        pos.x += vel.x * dt;
    }

    for (pos, vel) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        pos.y += vel.y * dt;
    }
}
