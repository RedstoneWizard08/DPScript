use super::Node;
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Call {
    /// The function we are calling
    pub function: Spanned<String>,

    /// The arguments
    pub args: Vec<Node>,

    /// The span
    pub span: SourceSpan,

    /// The parent object
    pub parent: Option<Spanned<String>>,
}
