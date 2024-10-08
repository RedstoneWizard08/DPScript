use super::Lowerable;
use crate::{CheckerContext, IRNode, LoweringContext, Result, Variable};

impl Lowerable for Variable {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        Ok(nodes)
    }
}
