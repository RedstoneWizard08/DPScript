use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// The name of the attribute.
    pub name: Spanned<String>,

    /// The attribute's value. Right now, this can only ever be a string.
    pub value: AttributeValue,

    /// The span.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeValue {
    String(Spanned<String>),
    Ident(Spanned<String>),
}
