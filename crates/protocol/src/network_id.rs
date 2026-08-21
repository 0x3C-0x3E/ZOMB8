use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash,
)]
pub struct ProtocolNetworkId(pub u32);
