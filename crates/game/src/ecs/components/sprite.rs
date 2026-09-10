use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sprite {
    pub id: String,
    pub rect: Rect,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rotation(pub f32);

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct SerRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<macroquad::math::Rect> for SerRect {
    fn from(rect: macroquad::math::Rect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
        }
    }
}

impl From<SerRect> for macroquad::math::Rect {
    fn from(rect: SerRect) -> Self {
        Self {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
        }
    }
}

impl Sprite {
    pub fn new(id: &str, rect: Rect) -> Self {
        Self {
            id: id.to_string(),
            rect,
        }
    }

    pub fn set_y(&mut self, y: u32) {
        self.rect.y = y as f32;
    }
}
