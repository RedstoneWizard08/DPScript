use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub is_pub: bool,
    pub name: Spanned<String>,
    pub span: SourceSpan,
    pub entries: Vec<Spanned<String>>,
}

impl Enum {
    pub fn entries(&self) -> Vec<String> {
        self.entries.iter().map(|(v, _)| v.clone()).collect()
    }
}
