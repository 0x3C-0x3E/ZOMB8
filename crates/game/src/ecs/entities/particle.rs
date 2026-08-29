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

pub struct TimeToLive(pub f32);

pub enum ParticleKind {
    DeathZombie,
    DeathPlayer,
    BulletCollision,
    BulletTrail,
}

impl ParticleKind {
    pub fn get_sprite_rect(&self) -> Rect {
        match self {
            Self::DeathZombie => Rect::new(16., 0., 8., 8.),
            Self::DeathPlayer => Rect::new(16., 0., 8., 8.),
            Self::BulletCollision => Rect::new(
                24.,
                if rand::rand().is_multiple_of(2) {
                    8.0
                } else {
                    0.0
                },
                8.,
                8.,
            ),
            Self::BulletTrail => Rect::new(
                32.,
                if rand::rand().is_multiple_of(2) {
                    8.0
                } else {
                    0.0
                },
                8.,
                8.,
            ),
        }
    }
}

impl Particle {
    pub fn spawn(world: &mut World, pos: Position, kind: &ParticleKind) -> Entity {
        world.spawn((
            Particle,
            TimeAlive(0.0),
            TimeToLive(rand::gen_range(0.2, 1.0)),
            pos,
            Velocity::from(
                Vec2::from_angle(rand::gen_range(0.0, 2.0 * PI)) * rand::gen_range(15.0, 50.0),
            ),
            Rotation(rand::gen_range(0.0, 360.0)),
            Sprite::new("particle", kind.get_sprite_rect()),
        ))
    }
}
