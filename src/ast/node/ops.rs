use super::Node;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Operation {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub kind: OperationKind,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
}
