pub mod node;

use crate::{module_indexer_add, Result};
pub use node::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct AST {
    pub nodes: Vec<Node>,
    pub modules: Option<HashMap<String, Module>>,
    pub top_level: Option<HashMap<String, Vec<TopLevelNode>>>,
    pub imports: Option<HashMap<String, Vec<Import>>>,
    pub funcs: Option<HashMap<String, Vec<Function>>>,
    pub vars: Option<HashMap<String, Vec<Variable>>>,
    pub blocks: Option<HashMap<String, Vec<Block>>>,
    pub enums: Option<HashMap<String, Vec<Enum>>>,
    pub objectives: Option<HashMap<String, Vec<Objective>>>,
    pub exports: Option<HashMap<String, Vec<Export>>>,
}

impl AST {
    pub fn merge(&mut self, other: AST) -> &mut Self {
        self.nodes.extend(other.nodes);
        self
    }

    pub fn collect_modules(&self) -> HashMap<String, Module> {
        let mut modules: HashMap<String, Module> = HashMap::new();

        for node in &self.nodes {
            if let Node::Module(module) = node {
                let name = module
                    .name
                    .iter()
                    .map(|v| v.0.clone())
                    .collect::<Vec<_>>()
                    .join("/");

                if let Some(it) = modules.get_mut(&name) {
                    it.body.extend(module.no_submodules());
                } else {
                    modules.insert(name.clone(), module.with_no_submodules());
                }

                for (name, item) in module.collect_submodules(name) {
                    if let Some(it) = modules.get_mut(&name) {
                        it.body.extend(item.body);
                    } else {
                        modules.insert(name, item);
                    }
                }
            }
        }

        modules
    }

    pub fn index_modules(&mut self) -> Result<&mut Self> {
        let mut modules = self.collect_modules();

        for (_, module) in &mut modules {
            module.index_top_level_nodes()?;
        }

        self.modules = Some(modules.clone());

        let mut top_level: HashMap<String, Vec<TopLevelNode>> = HashMap::new();
        let mut imports: HashMap<String, Vec<Import>> = HashMap::new();
        let mut funcs: HashMap<String, Vec<Function>> = HashMap::new();
        let mut vars: HashMap<String, Vec<Variable>> = HashMap::new();
        let mut blocks: HashMap<String, Vec<Block>> = HashMap::new();
        let mut enums: HashMap<String, Vec<Enum>> = HashMap::new();
        let mut objectives: HashMap<String, Vec<Objective>> = HashMap::new();
        let mut exports: HashMap<String, Vec<Export>> = HashMap::new();

        for (name, module) in modules {
            module_indexer_add!(top_level += (name, module));
            module_indexer_add!(imports += (name, module));
            module_indexer_add!(funcs += (name, module));
            module_indexer_add!(vars += (name, module));
            module_indexer_add!(blocks += (name, module));
            module_indexer_add!(enums += (name, module));
            module_indexer_add!(objectives += (name, module));
            module_indexer_add!(exports += (name, module));
        }

        self.top_level = Some(top_level);
        self.imports = Some(imports);
        self.funcs = Some(funcs);
        self.vars = Some(vars);
        self.blocks = Some(blocks);
        self.enums = Some(enums);
        self.objectives = Some(objectives);
        self.exports = Some(exports);

        Ok(self)
    }
}
