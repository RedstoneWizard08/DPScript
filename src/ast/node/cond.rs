use super::Node;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Conditional {
    /// The condition
    pub condition: Vec<Node>,

    /// The body
    pub body: Vec<Node>,

    /// If there is an else condition, this is its body
    pub else_body: Vec<Node>,

    /// The span
    pub span: SourceSpan,
}
