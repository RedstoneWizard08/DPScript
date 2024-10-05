use super::Node;
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
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
