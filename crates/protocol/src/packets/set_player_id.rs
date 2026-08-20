use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketPing {
    pub id: ProtocolNetworkId,
}

impl PacketPayload for PacketPing {
    const KIND: PacketKind = PacketKind::SetPlayerId;
}

impl PacketPing {
    pub fn new(id: ProtocolNetworkId) -> Self {
        Self { id }
    }
}
