use std::f32::consts::PI;

use glam::Vec2;
use hecs::{Entity, World};
use macroquad::{math::Rect, rand};
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{
        sprite::{Rotation, Sprite},
        transform::Position,
    },
    transform::Velocity,
};

#[derive(Serialize, Deserialize)]
pub struct Particle;

pub struct TimeAlive(pub f32);

impl Particle {
    pub fn spawn(world: &mut World, pos: Position) -> Entity {
        world.spawn((
            Particle,
            TimeAlive(0.0),
            pos,
            Velocity::from(
                Vec2::from_angle(rand::gen_range(0.0, 2.0 * PI)) * rand::gen_range(50.0, 100.0),
            ),
            Rotation(rand::gen_range(0.0, 360.0)),
            Sprite::new(
                "particle",
                Rect {
                    x: 24.0,
                    y: 0.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
        ))
    }
}
