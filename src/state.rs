use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    config::PackToml,
    expr::{function::Func, variable::Var, Expr},
    Result,
};

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
    pub fn new(config: PackToml, root: PathBuf) -> Self {
        Self {
            out_dir: root.join(&config.build.output),
            config,
            ..Default::default()
        }
    }

    pub fn update(
        &mut self,
        file: impl AsRef<str>,
        source: impl AsRef<str>,
        items: Vec<Expr>,
    ) -> Result<()> {
        self.file = file.as_ref().to_string();
        self.source = source.as_ref().to_string();

        let prefix = format!("{}:{}_", self.config.pack.namespace, self.file);

        for item in items {
            match item {
                Expr::Func(func) => {
                    self.functions.insert(
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
                        self.globals
                            .insert(var.name.clone(), (format!("{}{}", prefix, var.name), var));
                    }
                }

                _ => {}
            };
        }

        Ok(())
    }
}
