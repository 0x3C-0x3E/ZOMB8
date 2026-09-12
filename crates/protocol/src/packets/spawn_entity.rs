use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EntityKind {
    Tile,
    Player,
    Zombie,
    Bullet,
    HealthPack,
}

#[derive(Serialize, Deserialize)]
pub struct PacketSpawnEntity {
    pub network_id: ProtocolNetworkId,
    pub kind: EntityKind,
    pub pos: Vec2,
}

impl PacketPayload for PacketSpawnEntity {
    const KIND: PacketKind = PacketKind::SpawnEntity;
}

impl PacketSpawnEntity {
    pub fn new(network_id: ProtocolNetworkId, kind: EntityKind, pos: Vec2) -> Self {
        Self {
            network_id,
            kind,
            pos,
        }
    }
}
