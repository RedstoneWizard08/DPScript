use super::Lowerable;
use crate::{CheckerContext, Conditional, IRNode, LoweringContext, Result};

impl Lowerable for Conditional {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        Ok(nodes)
    }
}
