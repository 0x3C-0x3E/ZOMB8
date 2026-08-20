use crate::{
    ecs::components::{sprite::Sprite, transform::Position},
    game::{state::State, texture_manager::TextureManager},
};
use macroquad::prelude::*;

use hecs::Entity;

pub struct Camera {
    pub pos: Vec2,
    pub scale: f32,
}

pub struct RenderingState {
    pub camera: Camera,
}

static DEFAULT_SCALE: f32 = 5.0;

impl RenderingState {
    pub fn new() -> Self {
        Self {
            camera: Camera {
                pos: (0.0, 0.0).into(),
                scale: DEFAULT_SCALE,
            },
        }
    }
}

impl Default for RenderingState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn rendering_system(state: &mut State, texture_manager: &TextureManager) {
    let rd_state = &state.rendering_state;
    let camera = &rd_state.camera;

    clear_background(Color::from_hex(0x696353));

    for (_e, pos, sprite) in state.world.query::<(Entity, &Position, &Sprite)>().iter() {
        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: sprite.rect.w * camera.scale,
                y: sprite.rect.h * camera.scale,
            }),
            source: Some(sprite.rect),
            ..Default::default()
        };

        draw_texture_ex(
            texture_manager
                .get_texture(&sprite.id)
                .expect("invalid texture"),
            ((pos.x - camera.pos.x) * camera.scale) as i32 as f32,
            ((pos.y - camera.pos.y) * camera.scale) as i32 as f32,
            WHITE,
            params,
        );
    }
}
