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

macro_rules! node_from {
    ($other: ident = $field: ident) => {
        impl From<$other> for IRNode {
            fn from(value: $other) -> Self {
                Self::$field(value)
            }
        }
    };
}

node_from!(IRDefinition = Definition);
node_from!(IRDataOperation = DataOperation);
node_from!(IRFunction = Function);
node_from!(IRConcat = Concat);
node_from!(IRLiteral = Literal);
node_from!(IRArgumentOperation = Argument);
node_from!(IRTag = Tag);
node_from!(IRBlock = Block);
node_from!(IRCommand = Command);
node_from!(IRExecute = Execute);
node_from!(IRCall = Call);
node_from!(String = Reference);