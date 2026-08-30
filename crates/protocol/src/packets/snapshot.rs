use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
    packets::spawn_entity::EntityKind,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct PacketSnapshot {
    pub tick: u64,
    pub last_ack_seq: u32,
    pub entities: Vec<(ProtocolNetworkId, EntityState)>,
    pub entity_health: Vec<(ProtocolNetworkId, u32)>,
}

impl PacketPayload for PacketSnapshot {
    const KIND: PacketKind = PacketKind::Snapshot;
}

impl PacketSnapshot {
    pub fn new(
        tick: u64,
        last_ack_seq: u32,
        entities: Vec<(ProtocolNetworkId, EntityState)>,
        entity_health: Vec<(ProtocolNetworkId, u32)>,
    ) -> Self {
        Self {
            tick,
            last_ack_seq,
            entities,
            entity_health,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityState {
    pub kind: EntityKind,
    pub pos: Vec2,
    pub vel: Vec2,
}

impl EntityState {
    pub fn new(kind: EntityKind, pos: Vec2, vel: Vec2) -> Self {
        Self { kind, pos, vel }
    }
}
