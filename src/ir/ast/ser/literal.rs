use super::SerializeNode;
use crate::IRLiteral;

impl SerializeNode for IRLiteral {
    fn serialize_node(&self) -> String {
        match self {
            IRLiteral::String(s) => format!("'{}'", s),
            IRLiteral::PathOf(v) => format!("path!({})", v),
            IRLiteral::StoreOf(v) => format!("store!({})", v),
        }
    }
}
