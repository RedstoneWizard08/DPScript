use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    pub span: SourceSpan,
    pub kind: TypeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeKind {
    Int,
    Float,
    Bool,
    String,
    NBT,
    NBTPath,
    Identifier,
    Selector,
    Component,
    Storage,
    Objective,
    Ident(String),
    Array(Box<Type>),
}
