use miette::SourceSpan;
use serde::{Deserialize, Serialize};

use crate::util::Spanned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    /// This would be the `std/some/` part of `import std/some/{abc, def, ghi}`.
    pub base: Vec<Spanned<String>>,
    pub imports: Vec<ImportNode>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportNode {
    Object(String, SourceSpan),
    Module(Import),
}
