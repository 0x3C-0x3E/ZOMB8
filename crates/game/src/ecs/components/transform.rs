use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn update_vec2(&mut self, vec2: Vec2) {
        self.x = vec2.x;
        self.y = vec2.y;
    }
}

impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Position> for Vec2 {
    fn from(pos: Position) -> Self {
        pos.vec2()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct RenderPosition {
    pub x: f32,
    pub y: f32,
}

impl RenderPosition {
    pub fn lerp(&mut self, last: &Position, current: &Position, alpha: f32) {
        self.x = last.x + (current.x - last.x) * alpha;
        self.y = last.y + (current.y - last.y) * alpha;
    }

    pub fn from_pos(pos: Position) -> Self {
        Self { x: pos.x, y: pos.x }
    }

    pub fn to_pos(&self) -> Position {
        Position::new(self.x, self.y)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn update_vec2(&mut self, vec2: Vec2) {
        self.x = vec2.x;
        self.y = vec2.y;
    }
}

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<Velocity> for Vec2 {
    fn from(pos: Velocity) -> Self {
        pos.vec2()
    }
}
