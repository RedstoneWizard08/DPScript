use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInfo {
    pub pack: PackMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMeta {
    pub pack_format: u8,
    pub description: String,
}
