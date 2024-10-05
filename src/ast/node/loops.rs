use super::Node;
use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loop {
    /// The variable name for the loop entry
    pub var_name: Spanned<String>,

    /// The array we are looping through
    pub array: Spanned<String>,

    /// The span
    pub span: SourceSpan,

    /// The loop body
    pub body: Vec<Node>,
}
