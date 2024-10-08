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
mod tag;

pub use arg::*;
pub use block::*;
pub use call::*;
pub use cmd::*;
pub use concat::*;
pub use data::*;
pub use def::*;
pub use exec::*;
pub use func::*;
pub use literal::*;
pub use tag::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IRNode {
    Definition(IRDefinition),
    DataOperation(IRDataOperation),
    Function(IRFunction),
    Concat(IRConcat),
    Literal(IRLiteral),
    Argument(IRArgumentOperation),
    Tag(IRTag),
    Block(IRBlock),
    Command(IRCommand),
    Execute(IRExecute),
    Call(IRCall),
    Reference(String),
    // TODO: For each loops
}
