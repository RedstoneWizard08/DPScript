use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub name: String,
    pub ty: Option<String>,
    pub value: Box<Expr>,
    pub is_const: bool,
}
