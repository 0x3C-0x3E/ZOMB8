use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketSnapshot {
    pub tick: u64,
    pub last_ack_seq: u32,
    pub players: Vec<(ProtocolNetworkId, Vec2)>,
}

impl PacketPayload for PacketSnapshot {
    const KIND: PacketKind = PacketKind::Snapshot;
}

impl PacketSnapshot {
    pub fn new(tick: u64, last_ack_seq: u32, players: Vec<(ProtocolNetworkId, Vec2)>) -> Self {
        Self {
            tick,
            last_ack_seq,
            players,
        }
    }
}
