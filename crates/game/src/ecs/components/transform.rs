use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Position(Vec2);

impl Position {
    pub fn zero() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Velocity(Vec2);
