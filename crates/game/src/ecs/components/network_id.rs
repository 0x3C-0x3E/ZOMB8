use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct NetworkId(pub u32);
