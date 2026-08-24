use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketLevelData {
    pub tiles: Vec<(ProtocolNetworkId, Vec2)>,
    // TODO: pub sprite: SerializeRect
}

impl PacketPayload for PacketLevelData {
    const KIND: PacketKind = PacketKind::LevelData;
}

impl PacketLevelData {
    pub fn new(tiles: Vec<(ProtocolNetworkId, Vec2)>) -> Self {
        Self { tiles }
    }
}
