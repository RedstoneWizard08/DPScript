use std::fs;

use serde::{Deserialize, Serialize};

use crate::{expr::Expr, lines::LineBuilder, state::State, var::Var, Result, DPSCRIPT_RETURN_VAR};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Func {
    pub name: String,
    pub custom_name: Option<String>,
    pub args: Vec<(String, String)>,
    pub ret: Option<String>,
    pub body: Vec<Expr>,
}

impl Func {
    pub fn compile(&self, state: &mut State) -> Result<()> {
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
            b.push(expr.compile(state, DPSCRIPT_RETURN_VAR)?);
        }

        let fp = state
            .out_dir
            .join("data")
            .join(&state.config.pack.namespace)
            .join("functions")
            .join(format!(
                "{}_{}.mcfunction",
                state.file,
                self.custom_name.clone().unwrap_or(self.name.clone())
            ));

        let dir = fp.parent().unwrap();

        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }

        fs::write(fp, b.build())?;

        Ok(())
    }
}
