use crate::{
    call::Call, comp::Component, func::Func, nbt::Nbt, op::Operation, selector::Selector, var::Var,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Ident(String),
    String(String),
    Component(Component),
    Selector(Selector),
    Nbt(Nbt),
    Import(String),
    Call(Call),
    Command(Call),
    Func(Func),
    Var(Var),
    Operation(Operation),
    Array(Vec<Expr>),
    Return(Box<Expr>),

    None,
}

impl Expr {
    pub fn fix(&self) -> Self {
        match self.clone() {
            Expr::Call(mut call) => {
                call.args = call
                    .args
                    .iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect();

                Expr::Call(call)
            }

            Expr::Command(mut call) => {
                call.args = call
                    .args
                    .iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect();

                Expr::Command(call)
            }

            Expr::Func(mut func) => {
                func.body = func
                    .body
                    .iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect();

                Expr::Func(func)
            }

            Expr::Var(mut var) => {
                var.value = Box::new(var.value.fix());

                Expr::Var(var)
            }

            Expr::Component(mut comp) => {
                if let Some(expr) = &mut comp.from_expr {
                    *expr = Box::new(expr.fix());
                }

                Expr::Component(comp)
            }

            Expr::Operation(mut op) => {
                op.lhs = Box::new(op.lhs.fix());
                op.rhs = Box::new(op.rhs.fix());

                Expr::Operation(op)
            }

            Expr::Array(arr) => Expr::Array(
                arr.iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect(),
            ),

            Expr::Return(expr) => Expr::Return(Box::new(expr.fix())),

            v => v,
        }
    }
}
