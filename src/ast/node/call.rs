use super::Node;
use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    /// The function we are calling
    pub function: Spanned<String>,

    /// The arguments
    pub args: Vec<Node>,

    /// The span
    pub span: SourceSpan,
}
