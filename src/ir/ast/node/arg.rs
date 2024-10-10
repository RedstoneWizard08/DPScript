use super::IRNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IRArgumentOperation {
    /// Set an argument.
    Set(IRSetArgument),

    /// Get an argument by it's index.
    Get(IRGetArgument),

    /// Clear the arguments store
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRSetArgument {
    /// The index of the argument.
    pub index: usize,

    /// The value to set this argument to.
    pub value: Box<IRNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRGetArgument {
    /// The index of the argument.
    pub index: usize,

    /// The ID of the variable to insert the value into.
    pub var: String,
}
