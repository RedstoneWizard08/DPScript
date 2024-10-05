// TODO: conditionals, enum value, return

pub mod attr;
pub mod block;
pub mod call;
pub mod enums;
pub mod func;
pub mod import;
pub mod literal;
pub mod loops;
pub mod module;
pub mod ops;
pub mod ty;
pub mod var;

pub use attr::*;
pub use block::*;
pub use call::*;
pub use enums::*;
pub use func::*;
pub use import::*;
pub use literal::*;
pub use loops::*;
pub use module::*;
pub use ops::*;
pub use ty::*;
pub use var::*;

use crate::Spanned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Module(Module),
    Literal(Literal),
    Ident(Spanned<String>),
    Import(Import),
    Function(Function),
    Variable(Variable),
    Call(Call),
    Operation(Operation),
    Block(Block),
    Loop(Loop),
    Enum(Enum),
}
