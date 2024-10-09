#[macro_use]
extern crate tracing;

mod ast;
mod builtins;
mod cli;
mod error;
mod ir;
mod lexer;
mod lowerer;
mod lsp;
mod macros;
mod pack;
mod tokenizer;
mod util;
mod validator;

pub use ast::*;
pub use builtins::*;
pub use cli::*;
pub use error::*;
pub use ir::*;
pub use lexer::*;
pub use lowerer::*;
pub use lsp::*;
pub use pack::*;
pub use tokenizer::*;
pub use util::*;
pub use validator::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;
