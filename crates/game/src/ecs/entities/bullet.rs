use hecs::{Entity, World};
use macroquad::math::Rect;
use protocol::packets::spawn_entity::EntityKind;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{
        moveable::{CollisionMesh, Moveable},
        snapshot_sync::SnapshotSync,
        sprite::{Rotation, Sprite},
        transform::Position,
    },
    network_id::NetworkId,
    transform::{RenderPosition, Velocity},
};

#[derive(Serialize, Deserialize)]
pub struct Bullet;

pub struct LinkedPlayerId(pub NetworkId);

impl Bullet {
    pub fn spawn(
        world: &mut World,
        pos: Position,
        rotation: Option<f32>,
        network_id: NetworkId,
        player_id: NetworkId,
    ) -> Entity {
        world.spawn((
            Bullet,
            pos,
            RenderPosition::from_pos(pos),
            SnapshotSync(EntityKind::Bullet),
            EntityKind::Bullet,
            CollisionMesh::default(),
            Velocity::zero(),
            Moveable,
            network_id,
            LinkedPlayerId(player_id),
            Sprite::new(
                "bullet",
                Rect {
                    x: 8.0,
                    y: 0.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
            Rotation(rotation.unwrap_or(0.0)),
        ))
    }
}
