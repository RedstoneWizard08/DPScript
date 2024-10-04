use miette::SourceSpan;
use serde::{Deserialize, Serialize};

use super::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub span: SourceSpan,
    pub nodes: Vec<Node>,
}
