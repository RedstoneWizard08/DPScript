//! DPScript's Intermediate Representation, creatively named DataPack IR (DPIR).
//!
//! > !!!! WARNING !!!!
//! >
//! > DPIR is NOT designed for manual editing, and as such, it is NOT recommended to
//! > manually edit generated files. DPIR is NOT type-checked, validity-checked, or
//! > other means of catching errors, so anything that was not system-generated may
//! > have errors that can't be seen until runtime!

mod ast;
mod lexer;
mod tokenizer;

pub use ast::*;
pub use lexer::*;
pub use tokenizer::*;
