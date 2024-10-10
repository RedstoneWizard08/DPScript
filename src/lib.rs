#[macro_use]
extern crate tracing;

mod cli;
mod compiler;
mod dpscript;
mod error;
mod ir;
mod lsp;
mod macros;
mod pack;
mod util;

pub use cli::*;
pub use compiler::*;
pub use dpscript::*;
pub use error::*;
pub use ir::*;
pub use lsp::*;
pub use pack::*;
pub use util::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;
