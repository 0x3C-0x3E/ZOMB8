use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::packet::{PacketKind, PacketPayload};

#[derive(Serialize, Deserialize)]
pub struct PacketPing {
    pub now: u64,
}

impl PacketPayload for PacketPing {
    const KIND: PacketKind = PacketKind::Ping;
}

impl PacketPing {
    pub fn new() -> Self {
        Self {
            now: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}
