use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{sprite::Sprite, transform::Position},
    network_id::NetworkId,
};

#[derive(Serialize, Deserialize)]
pub struct Tile;

impl Tile {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            Tile,
            pos,
            network_id,
            Sprite::new(
                "tileset",
                Rect {
                    x: 80.0,
                    y: 16.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
        ))
    }
}
