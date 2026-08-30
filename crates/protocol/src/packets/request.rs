use serde::{Deserialize, Serialize};

use crate::packet::{PacketKind, PacketPayload};

#[derive(Serialize, Deserialize)]
pub enum RequestKind {
    PlayerId,
    LevelData,
    Ping,
    Wave,
}

#[derive(Serialize, Deserialize)]
pub struct PacketRequest {
    pub kind: RequestKind,
}

impl PacketRequest {
    pub fn new(kind: RequestKind) -> Self {
        Self { kind }
    }
}

impl PacketPayload for PacketRequest {
    const KIND: PacketKind = PacketKind::Request;
}
