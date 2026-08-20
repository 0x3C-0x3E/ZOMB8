use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Moveable;

#[derive(Debug, Default, Serialize)]
pub struct LastLookDirection(pub bool);

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// #[derive(Debug, Default)]
// pub struct Acceleration {
//     pub x: f32,
//     pub y: f32,
// }
