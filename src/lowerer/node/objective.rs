use super::Lowerable;
use crate::{CheckerContext, IRCommand, IRNode, LoweringContext, Objective, Result};

impl Lowerable for Objective {
    fn lower(
        &mut self,
        _cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        lcx.init_nodes.push(IRNode::Command(IRCommand {
            cmd: vec![
                IRNode::Literal("scoreboard".into()),
                IRNode::Literal("objectives".into()),
                IRNode::Literal("add".into()),
                IRNode::Literal(self.id.0.clone().into()),
                IRNode::Literal(self.criteria.0.clone().into()),
            ],
        }));

        Ok(Vec::new())
    }
}
