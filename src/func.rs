use serde::{Deserialize, Serialize};

use crate::{compiler::Compilable, expr::Expr, lines::LineBuilder, state::State, var::Var, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Func {
    pub name: String,
    pub custom_name: Option<String>,
    pub args: Vec<(String, String)>,
    pub ret: Option<String>,
    pub body: Vec<Expr>,
}

impl Compilable for Func {
    fn compile(&self, state: &mut State) -> Result<String> {
        let mut b = LineBuilder::new();

        for (k, v) in &self.args {
            state.locals.insert(
                k.clone(),
                (
                    format!("args.{}", k),
                    Var {
                        is_const: false,
                        name: k.clone(),
                        ty: Some(v.clone()),
                        value: Box::new(Expr::None),
                    },
                ),
            );
        }

        for expr in &self.body {
            b.push(expr.compile(state, "")?);
        }

        Ok(b.build())
    }
}
