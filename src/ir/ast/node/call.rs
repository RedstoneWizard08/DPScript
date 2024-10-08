use serde::{Deserialize, Serialize};

/// A call.
/// This exists so I can implement methods for it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRCall {
    /// The called function name.
    pub function: String,
}
