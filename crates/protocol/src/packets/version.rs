use serde::{Deserialize, Serialize};

use crate::packet::{PacketKind, PacketPayload};

#[derive(Serialize, Deserialize)]
pub struct PacketVersion {
    pub version_string: String,
}

impl PacketVersion {
    pub fn new(version_string: String) -> Self {
        Self { version_string }
    }
}

impl PacketPayload for PacketVersion {
    const KIND: PacketKind = PacketKind::Version;
}
