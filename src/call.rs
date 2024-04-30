use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub func: String,
    pub args: Vec<Expr>,
    pub is_command: bool,
}
