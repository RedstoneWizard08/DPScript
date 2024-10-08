use super::SerializeNode;
use crate::IRArgumentOperation;

impl SerializeNode for IRArgumentOperation {
    fn serialize_node(&self) -> String {
        match self {
            Self::Set(it) => format!("argument set: {}, {};", it.index, it.value.serialize_node()),
            Self::Get(it) => format!("argument get: {}, {};", it.index, it.var),
            Self::Clear => "argument clear;".into(),
        }
    }
}
