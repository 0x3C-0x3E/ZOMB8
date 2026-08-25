use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{
        animation::{AnimController, ZombieAnimState},
        moveable::Moveable,
        snapshot_sync::SnapshotSync,
        sprite::{Rotation, Sprite},
        transform::Position,
    },
    network_id::{self, NetworkId},
    transform::{RenderPosition, Velocity},
};

#[derive(Serialize, Deserialize)]
pub struct Bullet;

impl Bullet {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            Bullet,
            pos,
            RenderPosition::from_pos(pos),
            SnapshotSync,
            Velocity::zero(),
            Moveable,
            network_id,
            Sprite::new(
                "bullet",
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
            Rotation(0.0),
        ))
    }
}
