use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::components::{sprite::Sprite, transform::Position};

#[derive(Serialize, Deserialize)]
pub struct Tile;

impl Tile {
    pub fn spawn(world: &mut World, pos: Position) -> Entity {
        world.spawn((
            Tile,
            pos,
            Sprite::new(
                "tileset",
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
        ))
    }
}
