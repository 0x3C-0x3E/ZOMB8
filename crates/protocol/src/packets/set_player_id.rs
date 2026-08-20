use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketSetPlayerId {
    pub id: ProtocolNetworkId,
}

impl PacketPayload for PacketSetPlayerId {
    const KIND: PacketKind = PacketKind::SetPlayerId;
}

impl PacketSetPlayerId {
    pub fn new(id: ProtocolNetworkId) -> Self {
        Self { id }
    }
}
