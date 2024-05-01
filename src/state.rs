use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{config::PackToml, expr::Expr, func::Func, var::Var};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct State {
    pub file: String,
    pub source: String,
    pub config: PackToml,

    /// A map of function names to a tuple with their compiled
    /// name and the function definition.
    pub functions: HashMap<String, (String, Func)>,

    /// A map of global constant names to a tuple with their
    /// compiled name and the variable definition.
    pub globals: HashMap<String, (String, Var)>,

    /// A map of locals to a tuple with their compiled name
    /// and variable definition.
    pub locals: HashMap<String, (String, Var)>,
}

impl State {
    pub fn from_root(
        config: PackToml,
        file: impl AsRef<str>,
        source: impl AsRef<str>,
        items: Vec<Expr>,
    ) -> Self {
        let mut me = State::default();
        let prefix = format!("{}:{}_", config.pack.namespace, file.as_ref());

        me.file = file.as_ref().to_string();
        me.source = source.as_ref().to_string();
        me.config = config;

        for item in items {
            match item {
                Expr::Func(func) => {
                    me.functions.insert(
                        func.name.clone(),
                        (
                            func.custom_name
                                .clone()
                                .unwrap_or(format!("{}{}", prefix, func.name)),
                            func,
                        ),
                    );
                }

                Expr::Var(var) => {
                    if var.is_const {
                        me.globals
                            .insert(var.name.clone(), (format!("{}{}", prefix, var.name), var));
                    }
                }

                _ => {}
            };
        }

        me
    }
}
