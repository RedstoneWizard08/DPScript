use super::SerializeNode;
use crate::IRNode;

impl IRNode {
    pub fn serialize_node(&self) -> Option<String> {
        Some(match self {
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
            Self::Goto(it) => format!("goto: ${};", it),
            Self::None => return None,

            Self::Group(it) => it
                .iter()
                .filter_map(|v| v.serialize_node())
                .collect::<Vec<_>>()
                .join("\n"),
        })
    }
}
