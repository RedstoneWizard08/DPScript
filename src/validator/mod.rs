mod checker;
mod helpers;

pub use checker::*;
pub use helpers::*;

use crate::{Module, Result, AST};

#[derive(Debug, Clone)]
pub struct Validator {
    pub ast: AST,
}

impl Validator {
    pub fn new(ast: AST) -> Self {
        Self { ast }
    }

    pub fn validate(&mut self) -> Result<&mut Self> {
        self.ast.cache_values()?;

        let mut cx = self.ast.create_checker_context()?;
        let mut modules = cx.modules.clone();

        for (name, module) in &mut modules {
            module.get_imported_objects(&cx)?;

            cx.modules.insert(name.clone(), module.clone());
        }

        for (_, module) in &mut modules {
            Module::check(module, &mut cx)?;
        }

        self.ast.modules = Some(modules);

        Ok(self)
    }
}
