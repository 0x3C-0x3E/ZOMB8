use crate::game::state::TextureManager;
use macroquad::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sprite {
    pub id: String,
    pub rect: Rect,
}

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

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string();
    }

    pub fn get_texture<'a>(&self, texture_manager: &'a TextureManager) -> &'a Texture2D {
        texture_manager
            .get_texture(&self.id)
            .unwrap_or_else(|| panic!("invalid sprite: {}", self.id))
    }
}
