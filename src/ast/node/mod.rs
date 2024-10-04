pub mod attr;
pub mod func;
pub mod import;
pub mod literal;
pub mod module;
pub mod ty;

pub use attr::*;
pub use func::*;
pub use import::*;
pub use literal::*;
pub use module::*;
pub use ty::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Module(Module),
    Literal(Literal),
    Ident(String),
    Import(Import),
    Function(Function),
}
