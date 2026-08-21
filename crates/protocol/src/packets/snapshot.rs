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
    pub players: Vec<(ProtocolNetworkId, PlayerState)>,
}

impl PacketPayload for PacketSnapshot {
    const KIND: PacketKind = PacketKind::Snapshot;
}

impl PacketSnapshot {
    pub fn new(
        tick: u64,
        last_ack_seq: u32,
        players: Vec<(ProtocolNetworkId, PlayerState)>,
    ) -> Self {
        Self {
            tick,
            last_ack_seq,
            players,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl PlayerState {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self { pos, vel }
    }
}
