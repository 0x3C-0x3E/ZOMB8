use hecs::{Entity, World};
use macroquad::math::Rect;
use protocol::packets::spawn_entity::EntityKind;
use serde::{Deserialize, Serialize};

use crate::ecs::{
    components::{
        animation::{AnimController, ZombieAnimState},
        health::Health,
        moveable::{Collideable, CollisionMesh, Moveable},
        snapshot_sync::SnapshotSync,
        sprite::Sprite,
        transform::Position,
    },
    network_id::NetworkId,
    transform::{RenderPosition, Velocity},
};

#[derive(Serialize, Deserialize)]
pub struct Zombie;

pub struct ZombieSpawnTimer(pub f32);

impl Zombie {
    pub fn spawn(
        world: &mut World,
        pos: Position,
        network_id: NetworkId,
        max_health: Option<u32>,
    ) -> Entity {
        world.spawn((
            Zombie,
            ZombieSpawnTimer(4.0),
            Health::new(max_health.unwrap_or(50), max_health.unwrap_or(50)),
            pos,
            RenderPosition::from_pos(pos),
            SnapshotSync(EntityKind::Zombie),
            EntityKind::Zombie,
            Velocity::zero(),
            Moveable,
            Collideable,
            CollisionMesh::default(),
            network_id,
            Sprite::new(
                "zombie",
                Rect {
                    x: 0.0,
                    y: 8.0,
                    w: 8.0,
                    h: 8.0,
                },
            ),
            AnimController {
                tick: 0.0,
                frame: 0,
                state: Box::new(ZombieAnimState::default()),
            },
        ))
    }
}
