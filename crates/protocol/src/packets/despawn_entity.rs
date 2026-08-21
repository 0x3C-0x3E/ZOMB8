use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct PacketDespawnEntity {
    pub network_id: ProtocolNetworkId,
}

impl PacketPayload for PacketDespawnEntity {
    const KIND: PacketKind = PacketKind::DespawnEntity;
}

impl PacketDespawnEntity {
    pub fn new(network_id: ProtocolNetworkId) -> Self {
        Self { network_id }
    }
}
