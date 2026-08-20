use serde::{Deserialize, Serialize};

use crate::{
    network_id::ProtocolNetworkId,
    packet::{PacketKind, PacketPayload},
};

#[derive(Serialize, Deserialize)]
pub struct InputMap {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl InputMap {
    pub fn zero() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PacketInput {
    pub client_id: ProtocolNetworkId,
    pub input_map: InputMap,
}

impl PacketPayload for PacketInput {
    const KIND: PacketKind = PacketKind::Input;
}

impl PacketInput {
    pub fn new(client_id: ProtocolNetworkId, input_map: InputMap) -> Self {
        Self {
            client_id,
            input_map,
        }
    }
}
