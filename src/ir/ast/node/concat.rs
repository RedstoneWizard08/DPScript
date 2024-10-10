use super::IRNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRConcat {
    /// A list of items to concatenate.
    pub items: Vec<IRNode>,
}
