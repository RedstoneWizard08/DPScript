use super::Lowerable;
use crate::{CheckerContext, Conditional, IRNode, LoweringContext, Result};

impl Lowerable for Conditional {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.body.is_empty() && self.else_body.is_empty() {
            return Ok(Vec::new());
        }

        let mut nodes = Vec::new();

        Ok(nodes)
    }
}
