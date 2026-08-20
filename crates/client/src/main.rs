#![allow(clippy::new_without_default)]

use game::{
    ecs::{entities::tile::Tile, systems::rendering::rendering_system, transform::Position},
    game::{state::State, texture_manager::TextureManager},
};
use macroquad::prelude::*;

mod spawn_network_entity;

fn window_conf() -> Conf {
    Conf {
        window_title: "".to_owned(),
        window_width: 600.0 as i32,
        window_height: 600.0 as i32,
        window_resizable: true,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_default_filter_mode(FilterMode::Nearest);
    let mut state = State::new();

    let mut texture_manager = TextureManager::new();

    texture_manager
        .load_texture("assets/img/tileset.png", "tileset")
        .await;

    loop {
        rendering_system(&mut state, &texture_manager);
        next_frame().await;
    }
}
