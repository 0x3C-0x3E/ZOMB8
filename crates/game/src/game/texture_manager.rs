use std::collections::HashMap;

use macroquad::prelude::*;

pub struct TextureManager {
    textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub async fn load_game_textures() -> Self {
        let mut texture_manager = Self::new();

        texture_manager
            .load_texture("assets/img/tileset.png", "tileset")
            .await;

        texture_manager
            .load_texture("assets/img/spritesheet.png", "spritesheet")
            .await;

        texture_manager
            .load_texture("assets/img/spritesheet.png", "bullet")
            .await;

        texture_manager
            .load_texture("assets/img/player.png", "player")
            .await;

        texture_manager
            .load_texture("assets/img/zombie.png", "zombie")
            .await;

        texture_manager
    }

    pub async fn load_texture(&mut self, path: &str, id: &str) {
        self.textures
            .insert(id.to_string(), load_texture(path).await.unwrap());
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}
