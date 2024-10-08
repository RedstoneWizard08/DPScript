use super::IRNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRBlock {
    /// The block ID.
    /// Ex: `$block0: { ... }` would be `block0`.
    pub id: String,

    /// The block's body.
    pub body: Vec<IRNode>,
}
