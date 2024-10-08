use super::SerializeNode;
use crate::IRDataOperation;

impl SerializeNode for IRDataOperation {
    fn serialize_node(&self) -> String {
        match self {
            Self::Set(it) => format!("data set: {}, {};", it.var, it.value.serialize_node()),
            Self::Append(it) => format!("data append: {}, {};", it.var, it.value.serialize_node()),
            Self::Copy(it) => format!("data copy: {}, {};", it.source, it.target),
        }
    }
}
