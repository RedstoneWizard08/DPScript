use miette::SourceSpan;
use serde::{Deserialize, Serialize};

use super::{attr::Attribute, Node, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// The function attributes.
    pub attrs: Vec<Attribute>,

    /// The function name.
    pub name: String,

    /// A list of arguments.
    pub args: Vec<FunctionArg>,

    /// The function return type.
    pub ret: Option<Type>,

    /// Is this a facade function? (A function that directly references a command)
    /// If this is true, `attrs` will have a `#[cmd = ...]` attribute.
    pub facade: bool,

    /// Is this a compiler builtin?
    pub compiler: bool,

    /// The body of the function.
    pub body: Vec<Node>,

    /// The span.
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionArg {
    pub attrs: Vec<Attribute>,
    pub name: String,
    pub ty: Type,
    pub span: SourceSpan,
}
