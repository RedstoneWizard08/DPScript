pub mod ctx;
pub mod import;
pub mod node;

pub use ctx::*;

use crate::{Module, Result};

pub trait Checker<T> {
    fn check(module: &(String, Module), item: &mut T, cx: &CheckerContext) -> Result<()>;
}
