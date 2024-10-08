use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IRLiteral {
    /// A string.
    String(String),

    /// Get the path of a variable declaration's name.
    /// *Technically* this isn't a literal but it is for our purposes.
    PathOf(String),

    /// Get the store of a variable declaration's name.
    /// *Technically* this isn't a literal but it is for our purposes.
    StoreOf(String),
}
