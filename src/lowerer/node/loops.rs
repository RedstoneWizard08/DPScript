use super::Lowerable;
use crate::{CheckerContext, IRNode, Loop, LoweringContext, Result};

impl Lowerable for Loop {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        Ok(nodes)
    }
}
