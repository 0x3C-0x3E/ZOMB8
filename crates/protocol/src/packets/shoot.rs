use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketShoot {
    pub linked_player_id: ProtocolNetworkId,
    pub pos: Vec2,
    pub rotation: f32,
}

impl PacketPayload for PacketShoot {
    const KIND: PacketKind = PacketKind::Shoot;
}

impl PacketShoot {
    pub fn new(linked_player_id: ProtocolNetworkId, pos: Vec2, rotation: f32) -> Self {
        Self {
            linked_player_id,
            pos,
            rotation,
        }
    }
}
