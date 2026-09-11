use ::glam::Vec2;
use macroquad::prelude::*;

use crate::ecs::transform::Position;

pub struct Camera {
    pub pos: Vec2,
    pub scale: f32,
}

impl Camera {
    pub fn get_render_pos(&self, pos: &Position) -> (f32, f32) {
        (
            ((pos.x - self.pos.x) * self.scale) as i32 as f32,
            ((pos.y - self.pos.y) * self.scale) as i32 as f32,
        )
    }

    pub fn set_camera(&mut self, player_pos: Position) {
        let half_w = screen_width() / 2.0 / self.scale;
        let half_h = screen_height() / 2.0 / self.scale;

        let target_x = player_pos.x - half_w;
        let target_y = player_pos.y - half_h;

        let smoothing = 8.0;
        let dt = get_frame_time().min(1.0 / 30.0);
        let t = (1.0 - (-smoothing * dt).exp()).clamp(0.0, 1.0);

        self.pos.x += (target_x - self.pos.x) * t;
        self.pos.y += (target_y - self.pos.y) * t;
    }
}
