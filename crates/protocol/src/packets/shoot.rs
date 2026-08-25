use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketShoot {
    pub id: ProtocolNetworkId,
    pub pos: Vec2,
    pub rotation: f32,
}

impl PacketPayload for PacketShoot {
    const KIND: PacketKind = PacketKind::Shoot;
}

impl PacketShoot {
    pub fn new(id: ProtocolNetworkId, pos: Vec2, rotation: f32) -> Self {
        Self { id, pos, rotation }
    }
}
