use std::collections::HashMap;

use hecs::World;
use macroquad::prelude::*;

use crate::ecs::systems::rendering::RenderingState;

pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub async fn load_texture(&mut self, path: &str, id: &str) {
        self.textures
            .insert(id.to_string(), load_texture(path).await.unwrap());
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }
}

pub struct State {
    pub world: World,
    pub texture_manager: TextureManager,
    pub rendering_state: RenderingState,
}

impl State {
    pub async fn new() -> Self {
        let mut texture_manager = TextureManager::new();

        texture_manager
            .load_texture("assets/img/tileset_floor.png", "tileset_floor")
            .await;

        Self {
            world: World::new(),
            rendering_state: RenderingState::new(),
            texture_manager,
        }
    }
}
