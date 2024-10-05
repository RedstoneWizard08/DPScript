use serde::{Deserialize, Serialize};

use super::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub kind: OperationKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
}
