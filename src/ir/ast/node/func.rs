use super::IRNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRFunction {
    /// The namespaced ID of this function.
    pub id: String,

    /// The body of the function.
    pub body: Vec<IRNode>,
}
