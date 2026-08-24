use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{snapshot_sync::SnapshotSync, sprite::Sprite, transform::Position},
    network_id::NetworkId,
    transform::{RenderPosition, Velocity},
};

#[derive(Serialize, Deserialize)]
pub struct Zombie;

impl Zombie {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            Zombie,
            pos,
            RenderPosition::from_pos(pos),
            SnapshotSync,
            Velocity::zero(),
            network_id,
            Sprite::new(
                "zombie",
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
