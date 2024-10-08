use super::Lowerable;
use crate::{CheckerContext, IRCommand, IRLiteral, IRNode, LoweringContext, Objective, Result};

impl Lowerable for Objective {
    fn lower(&self, _cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        lcx.init_nodes.push(IRNode::Command(IRCommand {
            cmd: Box::new(IRNode::Literal(IRLiteral::String(format!(
                "scoreboard objectives add {} {}",
                self.id.0, self.criteria.0
            )))),
        }));

        Ok(Vec::new())
    }
}
