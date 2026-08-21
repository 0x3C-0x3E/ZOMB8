use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{sprite::Sprite, transform::Position},
    network_id::NetworkId,
    transform::{RenderPosition, Velocity},
};

#[derive(Serialize, Deserialize)]
pub struct Mole;

impl Mole {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            Mole,
            pos,
            RenderPosition::from_pos(pos),
            Velocity::zero(),
            network_id,
            Sprite::new(
                "mole",
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
