use crate::{
    ecs::{
        components::{health::Health, score::Score, sprite::Sprite},
        systems::rendering::RenderingState,
    },
    game::state::State,
};

use hecs::Entity;
use macroquad::{miniquad::window::show_mouse, prelude::*};

pub fn draw_ui(state: &mut State, rd_state: &RenderingState, player: Option<Entity>) {
    let player = if let Some(player) = player {
        player
    } else {
        return;
    };

    let score = state.world.get::<&Score>(player).unwrap();
    draw_fancy_text(rd_state, &format!("Score: {0}", score.0), 10.0, 50.0);

    let health = state.world.get::<&Health>(player).unwrap();
    draw_fancy_text(rd_state, &format!("Health: {0}", health.get()), 10.0, 70.0);

    let text = format!("Wave {0}", state.current_wave);
    let dimm = measure_text(text, Some(&rd_state.font), 40, 1.0);
    draw_fancy_text(
        rd_state,
        &format!("Wave {0}", state.current_wave),
        screen_width() / 2.0 - dimm.width / 2.0,
        50.0,
    );
}

fn draw_fancy_text(rd_state: &RenderingState, text: &str, x: f32, y: f32) {
    let params = TextParams {
        font: Some(&rd_state.font),
        font_size: 40,
        color: BLACK,
        ..Default::default()
    };
    draw_text_ex(
        text,
        x + rd_state.camera.scale,
        y + rd_state.camera.scale,
        params.clone(),
    );

    let mut params = params;
    params.color = WHITE;

    draw_text_ex(text, x, y, params.clone());
}

pub fn draw_mouse_cursor(rd_state: &RenderingState) {
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
