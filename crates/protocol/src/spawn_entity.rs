use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::network_id::ProtocolNetworkId;

#[derive(Serialize, Deserialize)]
pub enum EntityKind {
    Tile,
    Player,
}

#[derive(Serialize, Deserialize)]
pub struct PacketSpawnEntity {
    pub network_id: ProtocolNetworkId,
    pub kind: EntityKind,
    pub pos: Vec2,
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
