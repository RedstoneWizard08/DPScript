use std::fmt;

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

impl OperationKind {
    pub fn name(&self) -> String {
        match self {
            Self::Add => "add",
            Self::Subtract => "sub",
            Self::Multiply => "mul",
            Self::Divide => "div",
            Self::And => "and",
        }
        .into()
    }
}

impl fmt::Display for OperationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
