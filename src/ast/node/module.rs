use miette::SourceSpan;
use serde::{Deserialize, Serialize};

use crate::Spanned;

use super::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: Spanned<String>,
    pub span: SourceSpan,
    pub body: Vec<Node>,
}