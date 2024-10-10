use super::Lowerable;
use crate::{CheckerContext, IRFunction, IRNode, LoweringContext, Module, Result};

impl Lowerable for Module {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        self.get_imported_objects(cx)?;

        cx.cur_modules.push(self.clone());
        lcx.module = self.name();

        let mut nodes = Vec::new();

        lcx.init_nodes.clear();
        lcx.tick_nodes.clear();

        for node in &mut self.body {
            nodes.extend(node.lower(cx, lcx)?);
        }

        if !lcx.init_nodes.is_empty() {
            let init_id = format!(
                "{}:__dpscript_gen/{}/blocks/init/__gen_{}",
                lcx.namespace,
                self.name(),
                lcx.inits
            );

            let init_block = IRNode::Function(IRFunction {
                id: init_id.clone(),
                body: lcx.init_nodes.clone(),
            });

            nodes.push(init_block);
            lcx.init_names.push(init_id);
        }

        if !lcx.tick_nodes.is_empty() {
            let tick_id = format!(
                "{}:__dpscript_gen/{}/blocks/tick/__gen_{}",
                lcx.namespace,
                self.name(),
                lcx.ticks
            );

            let tick_block = IRNode::Function(IRFunction {
                id: tick_id.clone(),
                body: lcx.tick_nodes.clone(),
            });

            nodes.push(tick_block);
            lcx.tick_names.push(tick_id);
        }

        lcx.init_nodes.clear();
        lcx.tick_nodes.clear();

        lcx.module.clear();
        cx.cur_modules.pop();

        Ok(nodes)
    }
}
