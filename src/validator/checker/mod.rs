mod block;
mod call;
mod cond;
mod ctx;
mod enums;
mod func;
mod import;
mod loops;
mod module;
mod node;
mod objective;
mod ops;
mod ret;
mod sub;
mod var;

pub use ctx::*;

use crate::Result;

pub trait Checker<T> {
    fn check(item: &mut T, cx: &mut CheckerContext) -> Result<()>;
}
