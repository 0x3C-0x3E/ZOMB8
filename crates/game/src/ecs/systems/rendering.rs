use crate::{
    ecs::components::{
        sprite::Sprite,
        transform::{LastLookDirection, Position},
    },
    game::state::State,
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

pub fn rendering_system(state: &mut State) {
    let rd_state = &state.rendering_state;
    let camera = &rd_state.camera;

    clear_background(Color::from_hex(0x696353));

    for (e, pos, sprite) in state.world.query::<(Entity, &Position, &Sprite)>().iter() {
        let flip = if let Some(dir) = state.world.entity(e).unwrap().get::<&LastLookDirection>() {
            dir.0
        } else {
            false
        };

        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: sprite.rect.w * camera.scale,
                y: sprite.rect.h * camera.scale,
            }),
            source: Some(sprite.rect),
            flip_x: flip,
            ..Default::default()
        };

        draw_texture_ex(
            sprite.get_texture(&state.texture_manager),
            ((pos.x - camera.pos.x) * camera.scale) as i32 as f32,
            ((pos.y - camera.pos.y) * camera.scale) as i32 as f32,
            WHITE,
            params,
        );
    }
}
