use std::ops::{Deref, DerefMut};

use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Vec2);

impl Deref for Position {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderPosition(pub Vec2);

impl RenderPosition {
    pub fn render_pos_lerp(&mut self, last: &Position, current: &Position, alpha: f32) {
        self.x = last.x + (current.x - last.x) * alpha;
        self.y = last.y + (current.y - last.y) * alpha;
    }
}

impl Deref for RenderPosition {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RenderPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Position> for RenderPosition {
    fn from(p: Position) -> Self {
        RenderPosition(p.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity(pub Vec2);

impl Deref for Velocity {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Position(v)
    }
}

impl From<Vec2> for RenderPosition {
    fn from(v: Vec2) -> Self {
        RenderPosition(v)
    }
}

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Velocity(v)
    }
}

impl Position {
    pub fn vec2(&self) -> Vec2 {
        self.0
    }
}

impl RenderPosition {
    pub fn vec2(&self) -> Vec2 {
        self.0
    }
}

impl Velocity {
    pub fn vec2(&self) -> Vec2 {
        self.0
    }
}
