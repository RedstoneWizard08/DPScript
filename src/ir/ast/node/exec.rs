use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRExecute {
    /// The selector of the group to execute as.
    pub selector: String,

    /// The block to execute for each entity.
    pub block: String,
}
