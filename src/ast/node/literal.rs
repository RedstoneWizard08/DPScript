use serde::{Deserialize, Serialize};

use crate::util::Spanned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Int(Spanned<i64>),
    Float(Spanned<f64>),
    Bool(Spanned<bool>),
    String(Spanned<String>),
}
