use serde::{Deserialize, Serialize};

use crate::{util::Spanned, Token};

use super::Node;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Int(Spanned<i64>),
    Float(Spanned<f64>),
    Bool(Spanned<bool>),
    String(Spanned<String>),
    Component(Spanned<String>),
    Array(Spanned<Vec<Node>>),
    Identifier(Spanned<String>),
    Path(Spanned<String>),

    /// TODO: HashMap
    Nbt(Spanned<Vec<Spanned<Token>>>),
}
