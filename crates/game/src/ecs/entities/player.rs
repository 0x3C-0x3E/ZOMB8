use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{sprite::Sprite, transform::Position},
    network_id::NetworkId,
};

#[derive(Serialize, Deserialize)]
pub struct Player;

impl Player {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            Player,
            pos,
            network_id,
            Sprite::new(
                "player",
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
