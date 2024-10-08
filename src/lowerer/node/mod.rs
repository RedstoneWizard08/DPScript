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
mod var;

use super::LoweringContext;
use crate::{CheckerContext, IRNode, Result};

pub trait Lowerable {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>>;
}
