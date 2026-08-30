use serde::{Deserialize, Serialize};

use crate::packet::{PacketKind, PacketPayload};

#[derive(Serialize, Deserialize)]
pub struct PacketWave {
    pub number: u32,
}

impl PacketWave {
    pub fn new(number: u32) -> Self {
        Self { number }
    }
}

impl PacketPayload for PacketWave {
    const KIND: PacketKind = PacketKind::Wave;
}
