use super::{Node, Type};
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Variable {
    /// Is this public?
    /// This must not be true if this is a local variable.
    pub is_pub: bool,

    /// Is this constant?
    pub is_const: bool,

    /// Is this a function argument or a loop item?
    pub is_arg: bool,

    /// The name of the variable.
    pub name: Spanned<String>,

    /// The variable's type. If [`Self::is_arg`] is true, this will never be [`None`].
    pub ty: Option<Type>,

    /// This can only be `None` if this is a function argument.
    pub value: Option<Box<Node>>,

    /// The span.
    pub span: SourceSpan,
}
