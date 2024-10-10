mod block;
mod call;
mod cond;
mod func;
mod literal;
mod loops;
mod module;
mod node;
mod objective;
mod ops;
mod ret;
mod sub;
mod ty;
mod var;

use super::LoweringContext;
use crate::{CheckerContext, IRNode, Result};

pub trait Lowerable {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>>;
}

pub trait Valued {
    fn get_value(
        &mut self,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
        nodes: &mut Vec<IRNode>,
    ) -> Result<IRNode>;
}
