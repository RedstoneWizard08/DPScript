use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRCondition {
    /// The score to check in
    pub score: String,

    /// The if block
    pub if_block: String,

    /// The else block
    pub else_block: String,

    /// The join block
    pub join_block: String,
}
