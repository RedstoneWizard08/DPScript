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
            let mut b = LineBuilder::new();
            let mut args = Vec::new();

            for (i, arg) in self.args.iter().enumerate() {
                if arg.is_value() {
                    args.push(arg.compile(state, "")?);
                } else {
                    let temp = format!("__cmd_arg_{}", i);

                    b.push(arg.compile(state, &temp)?);

                    args.push(format!(
                        "{{\"storage\": \"{}\", \"nbt\": \"{}\", \"interpret\": true}}",
                        DPSCRIPT_VAR_STORE, temp
                    ));
                }
            }

            b.push(format!("{} {}", self.func, args.join(" ")));

            Ok(b.build())
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
                        }
                        .into());
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
                }
                .into())
            }
        }
    }
}
