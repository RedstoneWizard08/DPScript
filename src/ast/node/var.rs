use serde::{Deserialize, Serialize};

use crate::Spanned;

use super::{Node, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub is_const: bool,
    pub name: Spanned<String>,
    pub ty: Option<Type>,
    pub value: Box<Node>,
}
