use super::Node;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Return {
    /// The return value
    pub value: Option<Box<Node>>,

    /// The span
    pub span: SourceSpan,
}
