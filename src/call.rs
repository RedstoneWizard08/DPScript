use serde::{Deserialize, Serialize};

use crate::{
    compiler::Compilable, error::CompilationError, expr::Expr, lines::LineBuilder, source,
    state::State, Result, DPSCRIPT_RETURN_VAR, DPSCRIPT_TEMP_STORE, DPSCRIPT_VAR_STORE,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Call {
    pub func: String,
    pub args: Vec<Expr>,
    pub is_command: bool,
}

impl Compilable for Call {
    fn compile(&self, state: &mut State) -> Result<String> {
        if self.is_command {
            Ok(format!(
                "{} {}",
                self.func,
                self.args
                    .iter()
                    .map(|v| v.compile(state, "").unwrap())
                    .collect::<Vec<_>>()
                    .join(" ")
            ))
        } else {
            if let Some((name, func)) = state.functions.clone().get(&self.func) {
                let mut b = LineBuilder::new();

                for (i, (name, _)) in func.args.iter().enumerate() {
                    if let Some(arg) = self.args.get(i) {
                        if arg.is_value() {
                            b.push(format!(
                                "data modify storage {} args.{} set value {}",
                                DPSCRIPT_TEMP_STORE,
                                name,
                                arg.compile(state, "")?
                            ));
                        } else {
                            b.push(arg.compile(state, format!("args.{}", name))?);
                        }
                    } else {
                        return Err(CompilationError {
                            src: source!(state),
                            err: format!("Missing argument for call: {} (index {})", name, i),
                        });
                    }
                }

                b.push(format!("function {}", name));

                if func.ret.is_some() {
                    b.push(format!(
                        "data modify storage {} {} set from storage {} {}",
                        DPSCRIPT_TEMP_STORE,
                        DPSCRIPT_RETURN_VAR,
                        DPSCRIPT_VAR_STORE,
                        DPSCRIPT_RETURN_VAR
                    ));
                }

                Ok(b.build())
            } else {
                Err(CompilationError {
                    src: source!(state),
                    err: format!("Cannot find a function named {}!", self.func),
                })
            }
        }
    }
}
