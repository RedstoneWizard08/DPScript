use super::Node;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub is_init: bool,
    pub is_tick: bool,
    pub span: SourceSpan,
    pub body: Vec<Node>,
}
