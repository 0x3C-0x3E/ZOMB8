use crate::{
    ecs::{
        components::{
            health::Health,
            score::Score,
            sprite::{Rotation, Sprite},
            transform::Position,
        },
        transform::{RenderPosition, Velocity},
    },
    game::{state::State, texture_manager::TextureManager},
};

use hecs::Entity;
use macroquad::{miniquad::window::show_mouse, prelude::*};

pub struct Camera {
    pub pos: Vec2,
    pub scale: f32,
}

pub struct RenderingState {
    pub camera: Camera,
    pub texture_manager: TextureManager,
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
            texture_manager: TextureManager::load_game_textures().await,
            font: load_ttf_font("assets/font/super-mario-bros-nes.ttf")
                .await
                .unwrap(),
        }
    }

    pub fn get_render_pos(&self, pos: &Position) -> (f32, f32) {
        let camera = &self.camera;

        (
            ((pos.x - camera.pos.x) * camera.scale) as i32 as f32,
            ((pos.y - camera.pos.y) * camera.scale) as i32 as f32,
        )
    }

    pub fn set_camera(&mut self, player_pos: Position) {
        let half_w = screen_width() / 2.0 / self.camera.scale;
        let half_h = screen_height() / 2.0 / self.camera.scale;

        let target_x = player_pos.x - half_w;
        let target_y = player_pos.y - half_h;

        let smoothing = 8.0;
        let dt = get_frame_time().min(1.0 / 30.0);
        let t = (1.0 - (-smoothing * dt).exp()).clamp(0.0, 1.0);

        self.camera.pos.x += (target_x - self.camera.pos.x) * t;
        self.camera.pos.y += (target_y - self.camera.pos.y) * t;
    }
}

pub fn rendering_system(state: &mut State, player: Option<Entity>, rd_state: &RenderingState) {
    let camera = &rd_state.camera;

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
}

fn draw_ui(state: &mut State, rd_state: &RenderingState, player: Option<Entity>) {
    let player = if let Some(player) = player {
        player
    } else {
        return;
    };

    let score = state.world.get::<&Score>(player).unwrap();
    let params_bg = TextParams {
        font: Some(&rd_state.font),
        font_size: 20,
        color: BLACK,
        ..Default::default()
    };
    let mut params = params_bg.clone();
    params.color = WHITE;

    draw_text_ex(
        format!("Score: {0}", score.0),
        10.0,
        50.0,
        params_bg.clone(),
    );
    draw_text_ex(
        format!("Score: {0}", score.0),
        10.0 - rd_state.camera.scale,
        50.0 - rd_state.camera.scale,
        params.clone(),
    );

    let health = state.world.get::<&Health>(player).unwrap();

    draw_text_ex(
        format!("Health: {0}", health.get()),
        10.0,
        70.0,
        params_bg.clone(),
    );
    draw_text_ex(
        format!("Health: {0}", health.get()),
        10.0 - rd_state.camera.scale,
        70.0 - rd_state.camera.scale,
        params.clone(),
    );
}

fn draw_mouse_cursor(rd_state: &RenderingState) {
    let camera = &rd_state.camera;

    show_mouse(false);

    let sprite = Sprite {
        id: "spritesheet".to_string(),
        rect: Rect::new(64.0, 24.0, 8.0, 8.0),
    };

    let params = DrawTextureParams {
        dest_size: Some(Vec2 {
            x: sprite.rect.w * camera.scale,
            y: sprite.rect.h * camera.scale,
        }),
        source: Some(sprite.rect),
        ..Default::default()
    };
    draw_texture_ex(
        rd_state
            .texture_manager
            .get_texture(&sprite.id)
            .expect("invalid texture"),
        mouse_position().0,
        mouse_position().1,
        WHITE,
        params,
    );
}
