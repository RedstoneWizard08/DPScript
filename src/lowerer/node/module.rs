use super::Lowerable;
use crate::{CheckerContext, IRFunction, IRNode, LoweringContext, Module, Result};

impl Lowerable for Module {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        lcx.module = self.name();

        let mut nodes = Vec::new();

        lcx.init_nodes.clear();
        lcx.tick_nodes.clear();

        for node in &self.body {
            nodes.extend(node.lower(cx, lcx)?);
        }

        let init_id = format!(
            "{}:__dpscript_gen/{}/blocks/init/{}",
            lcx.namespace,
            self.name(),
            lcx.inits
        );
        let tick_id = format!(
            "{}:__dpscript_gen/{}/blocks/tick/{}",
            lcx.namespace,
            self.name(),
            lcx.ticks
        );

        let init_block = IRNode::Function(IRFunction {
            id: init_id.clone(),
            body: lcx.init_nodes.clone(),
        });

        let tick_block = IRNode::Function(IRFunction {
            id: tick_id.clone(),
            body: lcx.tick_nodes.clone(),
        });

        nodes.push(init_block);
        nodes.push(tick_block);

        lcx.init_names.push(init_id);
        lcx.tick_names.push(tick_id);

        lcx.init_nodes.clear();
        lcx.tick_nodes.clear();

        lcx.module.clear();

        Ok(nodes)
    }
}
