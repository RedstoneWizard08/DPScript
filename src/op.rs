use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub lhs: Box<Expr>,
    pub op: String,
    pub rhs: Box<Expr>,
}
