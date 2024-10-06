pub mod checker;

pub use checker::*;

use crate::{Node, Result, AST};

#[derive(Debug, Clone)]
pub struct Validator {
    pub ast: AST,
}

impl Validator {
    pub fn new(ast: AST) -> Self {
        Self { ast }
    }

    pub fn validate(&mut self) -> Result<&mut Self> {
        let ast = self.ast.index_modules()?.clone();
        let mut modules = ast.modules.unwrap();
        let imports = ast.imports.unwrap();
        let funcs = ast.funcs.unwrap();
        let vars = ast.vars.unwrap();
        let blocks = ast.blocks.unwrap();
        let enums = ast.enums.unwrap();
        let objectives = ast.objectives.unwrap();
        let exports = ast.exports.unwrap();

        let cx = CheckerContext {
            modules: modules.clone(),
            imports,
            funcs,
            vars,
            blocks,
            enums,
            objectives,
            exports,
        };

        for module in &mut modules {
            let m = (module.0.clone(), module.1.clone());

            for node in &mut module.1.body {
                Node::check(&m, node, &cx)?;
            }
        }

        self.ast.modules = Some(modules);

        Ok(self)
    }
}
