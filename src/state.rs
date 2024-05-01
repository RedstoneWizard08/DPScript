use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::{config::PackToml, expr::Expr, func::Func, var::Var, Result};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct State {
    pub file: String,
    pub source: String,
    pub config: PackToml,
    pub out_dir: PathBuf,

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
    ) -> Result<Self> {
        let mut me = State::default();
        let prefix = format!("{}:{}_", config.pack.namespace, file.as_ref());

        let out_dir = PathBuf::from(file.as_ref())
            .parent()
            .unwrap()
            .join(&config.build.output);

        if !out_dir.exists() {
            fs::create_dir_all(&out_dir)?;
        }

        me.file = file.as_ref().to_string();
        me.source = source.as_ref().to_string();
        me.config = config;
        me.out_dir = out_dir;

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

        Ok(me)
    }
}
