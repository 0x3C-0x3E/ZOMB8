use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct PacketSnapshot {
    pub tick: u64,
    pub last_ack_seq: u32,
    pub entities: Vec<(ProtocolNetworkId, EntityState)>,
}

impl PacketPayload for PacketSnapshot {
    const KIND: PacketKind = PacketKind::Snapshot;
}

impl PacketSnapshot {
    pub fn new(
        tick: u64,
        last_ack_seq: u32,
        entities: Vec<(ProtocolNetworkId, EntityState)>,
    ) -> Self {
        Self {
            tick,
            last_ack_seq,
            entities,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntityState {
    pub pos: Vec2,
}

impl EntityState {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}
