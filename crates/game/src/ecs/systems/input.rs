use macroquad::prelude::*;

use crate::{
    ecs::{components::transform::Velocity, entities::player::Player, network_id::NetworkId},
    game::state::State,
};

pub fn input_system(state: &mut State, player_id: NetworkId) {
    for (id, vel) in state
        .world
        .query_mut::<(&NetworkId, &mut Velocity)>()
        .with::<&Player>()
    {
        if *id != player_id {
            continue;
        }

        let mut dir = Vec2::new(0.0, 0.0);
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dir.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dir.x += 1.0;
        }

        dir = dir.normalize_or_zero();
        vel.x = dir.x * 100.0;
        vel.y = dir.y * 100.0;
    }
}
