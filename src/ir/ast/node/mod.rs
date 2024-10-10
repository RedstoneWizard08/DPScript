mod arg;
mod block;
mod call;
mod cmd;
mod concat;
mod condition;
mod data;
mod def;
mod exec;
mod func;
mod literal;
mod tag;

use std::fmt;

pub use arg::*;
pub use block::*;
pub use call::*;
pub use cmd::*;
pub use concat::*;
pub use condition::*;
pub use data::*;
pub use def::*;
pub use exec::*;
pub use func::*;
pub use literal::*;
pub use tag::*;

use serde::{Deserialize, Serialize};

// TODO: For each loops
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
    Condition(IRCondition),
    Goto(String),

    /// Groups are ONLY used during the second pass (finalizing)
    Group(Vec<IRNode>),

    /// This is ONLY used during the second pass (finalizing), and is
    /// for when a node is replaced with nothing.
    None,
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
node_from!(IRCondition = Condition);
node_from!(String = Reference);

impl fmt::Display for IRNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definition(_) => write!(f, "definition"),
            Self::DataOperation(_) => write!(f, "data operation"),
            Self::Function(_) => write!(f, "function"),
            Self::Concat(_) => write!(f, "concatenation"),
            Self::Literal(_) => write!(f, "literal"),
            Self::Argument(_) => write!(f, "argument"),
            Self::Tag(_) => write!(f, "tag"),
            Self::Block(_) => write!(f, "block"),
            Self::Command(_) => write!(f, "command"),
            Self::Execute(_) => write!(f, "execution"),
            Self::Call(_) => write!(f, "call"),
            Self::Reference(_) => write!(f, "reference"),
            Self::Condition(_) => write!(f, "condition"),
            Self::Goto(_) => write!(f, "goto"),
            Self::Group(_) => write!(f, "group"),
            Self::None => write!(f, "none"),
        }
    }
}
