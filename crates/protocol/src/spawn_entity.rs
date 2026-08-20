use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::network_id::ProtocolNetworkId;

#[derive(Serialize, Deserialize)]
pub enum EntityKind {
    Tile,
    Player,
}

pub struct SpawnEntity {
    pub network_id: ProtocolNetworkId,
    pub kind: EntityKind,
    pub pos: Vec2,
}
