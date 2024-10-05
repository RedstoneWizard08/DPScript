use super::ImportNode;
use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    /// Do we export everything? (This is `export whatever/*;`)
    pub all: bool,

    /// The span
    pub span: SourceSpan,

    /// The module to export from
    pub module: Vec<Spanned<String>>,

    /// The items to export (empty if [`Self::all`] is true)
    pub items: Vec<ImportNode>,
}
