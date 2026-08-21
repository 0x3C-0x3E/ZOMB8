use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
    packets::spawn_entity::EntityKind,
};

#[derive(Serialize, Deserialize)]
pub struct PacketDespawnEntity {
    pub network_id: ProtocolNetworkId,
    pub kind: EntityKind,
    pub pos: Vec2,
}

impl PacketPayload for PacketDespawnEntity {
    const KIND: PacketKind = PacketKind::DespawnEntity;
}

impl PacketDespawnEntity {
    pub fn new(network_id: ProtocolNetworkId, kind: EntityKind, pos: Vec2) -> Self {
        Self {
            network_id,
            kind,
            pos,
        }
    }
}
