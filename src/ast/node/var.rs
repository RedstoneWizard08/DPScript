use super::{Node, Type};
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Variable {
    /// Is this public?
    /// This must not be true if this is a local variable.
    pub is_pub: bool,

    pub is_const: bool,
    pub name: Spanned<String>,
    pub ty: Option<Type>,
    pub value: Box<Node>,
    pub span: SourceSpan,
}
