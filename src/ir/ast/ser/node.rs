use super::SerializeNode;
use crate::IRNode;

impl SerializeNode for IRNode {
    fn serialize_node(&self) -> String {
        match self {
            Self::Argument(it) => it.serialize_node(),
            Self::Block(it) => it.serialize_node(),
            Self::Call(it) => it.serialize_node(),
            Self::Command(it) => it.serialize_node(),
            Self::Concat(it) => it.serialize_node(),
            Self::Definition(it) => it.serialize_node(),
            Self::DataOperation(it) => it.serialize_node(),
            Self::Function(it) => it.serialize_node(),
            Self::Literal(it) => it.serialize_node(),
            Self::Tag(it) => it.serialize_node(),
            Self::Execute(it) => it.serialize_node(),
            Self::Condition(it) => it.serialize_node(),
            Self::Reference(it) => it.clone(),
            Self::Goto(it) => format!("goto: {};", it),
        }
    }
}
