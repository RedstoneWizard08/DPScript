#[macro_use]
extern crate tracing;

pub mod ast;
pub mod error;
pub mod lexer;
pub mod macros;
pub mod tokenizer;
pub mod util;

pub use ast::*;
pub use error::*;
pub use lexer::*;
pub use tokenizer::*;
pub use util::*;

pub type ParserResult<T, E = ParserError> = Result<T, E>;
