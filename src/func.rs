use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    pub name: String,
    pub custom_name: Option<String>,
    pub args: Vec<(String, String)>,
    pub ret: Option<String>,
    pub body: Vec<Expr>,
}
