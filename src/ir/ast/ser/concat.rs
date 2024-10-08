use crate::IRConcat;

use super::SerializeNode;

impl SerializeNode for IRConcat {
    fn serialize_node(&self) -> String {
        self.items
            .iter()
            .map(|v| v.serialize_node())
            .collect::<Vec<_>>()
            .join(" + ")
    }
}
