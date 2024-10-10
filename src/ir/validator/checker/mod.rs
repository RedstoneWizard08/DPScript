mod arg;
mod block;
mod call;
mod cmd;
mod concat;
mod cond;
mod data;
mod def;
mod exec;
mod func;
mod literal;
mod node;
mod tag;

use super::IRCheckerContext;
use crate::Result;

pub trait IRChecker {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()>;
}
