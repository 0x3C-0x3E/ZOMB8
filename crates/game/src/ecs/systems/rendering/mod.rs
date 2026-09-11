use crate::ecs::systems::rendering::camera::Camera;
use crate::ecs::systems::rendering::shader::AvailableShaders;
use crate::ecs::systems::rendering::ui::draw_mouse_cursor;
use crate::ecs::systems::rendering::ui::draw_ui;
use crate::{
    ecs::{
        components::{
            health::Health,
            sprite::{Rotation, Sprite},
            transform::Position,
        },
        transform::{RenderPosition, Velocity},
    },
    game::{state::State, texture_manager::TextureManager},
};

use hecs::Entity;
use macroquad::prelude::*;

use shader::ShaderState;

mod camera;
mod shader;
mod ui;

pub struct ShakeState {
    pub active: bool,
    pub timer: f32,
    pub intensity: f32,
}

impl ShakeState {
    pub fn new() -> Self {
        Self {
            active: false,
            timer: 0.0,
            intensity: 0.0,
        }
    }

    pub fn set_shake(&mut self, duration: f32, intesity: f32) {
        self.timer = duration;
        self.intensity = intesity;
        self.active = true;
    }

    pub fn update(&mut self) {
        self.timer -= get_frame_time();
        if self.timer <= 0.0 {
            self.active = false;
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        if !self.active {
            (0.0, 0.0)
        } else {
            (
                rand::gen_range(-self.intensity, self.intensity),
                rand::gen_range(-self.intensity, self.intensity),
            )
        }
    }
}

pub struct RenderingState {
    pub camera: Camera,
    pub shake_state: ShakeState,
    pub texture_manager: TextureManager,
    pub shader_materials: ShaderState,
    pub font: Font,
}

static DEFAULT_SCALE: f32 = 4.0;

impl RenderingState {
    pub async fn new() -> Self {
        Self {
            camera: Camera {
                pos: (0.0, 0.0).into(),
                scale: DEFAULT_SCALE,
            },
            shake_state: ShakeState::new(),
            shader_materials: ShaderState::load().expect("could not load shaders"),
            texture_manager: TextureManager::load_game_textures().await,
            font: load_ttf_font("assets/font/font.ttf").await.unwrap(),
        }
    }
}

pub fn rendering_system(state: &mut State, player: Option<Entity>, rd_state: &mut RenderingState) {
    let camera = &rd_state.camera;

    let sh_state = &mut rd_state.shader_materials;

    sh_state.check_screen_changed();
    sh_state.set_camera();

    clear_background(Color::from_hex(0x727272));

    for (e, pos, sprite) in state.world.query::<(Entity, &Position, &Sprite)>().iter() {
        let mut render_pos = *pos;
        if let Ok(rpos) = state.world.get::<&RenderPosition>(e) {
            render_pos = rpos.to_pos();
        }

        let mut rotation: f32 = 0.0;
        if let Ok(rot) = state.world.get::<&Rotation>(e) {
            rotation = rot.0;
            if let Ok(vel) = state.world.get::<&Velocity>(e) {
                rotation = vel.y.atan2(vel.x);
            }
        }

        let flip = state
            .world
            .get::<&Velocity>(e)
            .map(|vel| vel.x < 0.0)
            .unwrap_or_default();

        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: sprite.rect.w * camera.scale,
                y: sprite.rect.h * camera.scale,
            }),
            source: Some(sprite.rect),
            flip_x: flip,
            rotation,
            ..Default::default()
        };

        draw_texture_ex(
            rd_state
                .texture_manager
                .get_texture(&sprite.id)
                .expect("invalid texture"),
            ((render_pos.x - camera.pos.x) * camera.scale) as i32 as f32,
            ((render_pos.y - camera.pos.y) * camera.scale) as i32 as f32,
            WHITE,
            params,
        );

        if let Ok(health) = state.world.get::<&Health>(e) {
            if health.health == health.max {
                continue;
            }

            draw_rectangle(
                ((render_pos.x - camera.pos.x) * camera.scale) as i32 as f32,
                ((render_pos.y - camera.pos.y - 4.0) * camera.scale) as i32 as f32,
                8.0 * camera.scale,
                2.0 * camera.scale,
                Color::from_hex(0x291e31),
            );
            draw_rectangle(
                ((render_pos.x - camera.pos.x) * camera.scale) as i32 as f32,
                ((render_pos.y - camera.pos.y - 4.0) * camera.scale) as i32 as f32,
                8.0 * camera.scale * (health.get() as f32 / health.get_max() as f32),
                2.0 * camera.scale,
                Color::from_hex(0xea4a6e),
            );
        }
    }

    draw_ui(state, rd_state, player);
    draw_mouse_cursor(rd_state);

    let sh_state = &mut rd_state.shader_materials;
    set_default_camera();

    sh_state.set_shader(AvailableShaders::CrtMaterial);

    rd_state.shake_state.update();
    let pos = rd_state.shake_state.get_pos();

    draw_texture_ex(
        &sh_state.render_target.texture,
        pos.0,
        pos.1,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            flip_y: true,
            ..Default::default()
        },
    );

    gl_use_default_material();
}
