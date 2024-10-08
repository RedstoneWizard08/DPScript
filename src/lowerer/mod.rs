mod ctx;
mod node;
mod remap;

pub use ctx::*;
pub use node::*;
pub use remap::*;

use crate::{Error, IRAst, IRNode, IRTag, Result, AST};

#[derive(Debug, Clone)]
pub struct Lowerer {
    pub ns: String,
    pub ast: AST,
    pub lowered: Option<IRAst>,
}

impl Lowerer {
    pub fn new(ns: impl AsRef<str>, ast: AST) -> Self {
        Self {
            ns: ns.as_ref().into(),
            ast,
            lowered: None,
        }
    }

    pub fn run(mut self) -> Result<Self> {
        self.ast.indexed = false;
        self.ast.cached = false;
        self.ast.cache_values()?;

        let mut cx = self.ast.create_checker_context()?;
        let mut lcx = LoweringContext::new(&self.ns);
        let mut nodes = Vec::new();
        let mut modules = cx.modules.clone();

        for (_, item) in &mut modules {
            nodes.extend(item.lower(&mut cx, &mut lcx)?);
        }

        nodes.push(IRNode::Tag(IRTag {
            name: "minecraft:tags/functions/load".into(),
            entries: lcx.init_names,
        }));

        nodes.push(IRNode::Tag(IRTag {
            name: "minecraft:tags/functions/tick".into(),
            entries: lcx.tick_names,
        }));

        self.lowered = Some(IRAst { nodes });

        Ok(self)
    }

    pub fn get_code(&self) -> Result<String> {
        if let Some(it) = &self.lowered {
            Ok(it.serialize_nodes())
        } else {
            Err(Error::Basic("The lowerer has not been run yet!".into()))
        }
    }
}
