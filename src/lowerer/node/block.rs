use super::Lowerable;
use crate::{Block, CheckerContext, IRFunction, IRNode, LoweringContext, Result};

impl Lowerable for Block {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let name = if self.is_init { "init" } else { "tick" };
        let index = if self.is_init { lcx.inits } else { lcx.ticks };
        let id = format!(
            "{}:__dpscript_gen/{}/blocks/{}/{}",
            lcx.namespace, lcx.module, name, index
        );

        let mut body = Vec::new();

        for item in &self.body {
            body.extend(item.lower(cx, lcx)?);
        }

        Ok(vec![IRNode::Function(IRFunction { id, body })])
    }
}
