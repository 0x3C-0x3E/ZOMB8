use hecs::{Entity, World};
use macroquad::math::Rect;
use protocol::packets::spawn_entity::EntityKind;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{
        moveable::CollisionMesh, snapshot_sync::SnapshotSync, sprite::Sprite, transform::Position,
    },
    network_id::NetworkId,
    transform::Velocity,
};

#[derive(Serialize, Deserialize)]
pub struct HealthPack;

impl HealthPack {
    pub fn spawn(world: &mut World, pos: Position, network_id: NetworkId) -> Entity {
        world.spawn((
            HealthPack,
            pos,
            Velocity::zero(),
            network_id,
            SnapshotSync(EntityKind::HealthPack),
            EntityKind::HealthPack,
            CollisionMesh::default(),
            Sprite::new("spritesheet", Rect::new(64.0, 16.0, 8.0, 8.0)),
        ))
    }
}
