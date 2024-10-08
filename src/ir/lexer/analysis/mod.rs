mod arg;
mod block;
mod call;
mod cmd;
mod concat;
mod data;
mod def;
mod exec;
mod func;
mod literal;
mod node;
mod tag;

use super::IRTokenCursor;
use crate::{IRNode, IRToken, Result, Spanned};

pub trait Analyzer<T> {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        nodes: &mut Vec<IRNode>,
    ) -> Result<Option<T>>;
}
