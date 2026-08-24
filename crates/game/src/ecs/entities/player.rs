use hecs::{Entity, World};
use macroquad::math::Rect;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{
        animation::{AnimController, PlayerAnimState},
        moveable::Moveable,
        snapshot_sync::SnapshotSync,
        sprite::Sprite,
        transform::Position,
    },
    network_id::NetworkId,
    transform::{RenderPosition, Velocity},
};

#[derive(Serialize, Deserialize)]
pub struct Player;

impl Player {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            Player,
            pos,
            RenderPosition::from_pos(pos),
            Velocity::zero(),
            SnapshotSync,
            network_id,
            Moveable,
            Sprite::new(
                "player",
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
            AnimController {
                tick: 0.0,
                frame: 0,
                state: Box::new(PlayerAnimState::default()),
            },
        ))
    }
}
