use macroquad::prelude::*;
use protocol::packets::input::InputMap;

use crate::{
    ecs::{components::transform::Velocity, entities::player::Player, network_id::NetworkId},
    game::state::State,
};

pub fn get_input_map() -> InputMap {
    let mut input_map = InputMap::zero();
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        input_map.up = true;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        input_map.down = true;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        input_map.left = true;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        input_map.right = true;
    }

    input_map
}

pub fn input_system(state: &mut State, player_id: NetworkId, input_map: &InputMap) {
    for (id, vel) in state
        .world
        .query_mut::<(&NetworkId, &mut Velocity)>()
        .with::<&Player>()
    {
        if *id != player_id {
            continue;
        }

        let mut dir = Vec2::new(0.0, 0.0);
        if input_map.up {
            dir.y -= 1.0;
        }
        if input_map.down {
            dir.y += 1.0;
        }
        if input_map.left {
            dir.x -= 1.0;
        }
        if input_map.right {
            dir.x += 1.0;
        }

        dir = dir.normalize_or_zero();
        vel.x = dir.x * 100.0;
        vel.y = dir.y * 100.0;
    }
}
