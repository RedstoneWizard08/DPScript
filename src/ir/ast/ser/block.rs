use super::SerializeNode;
use crate::{IRBlock, Indented};

impl SerializeNode for IRBlock {
    fn serialize_node(&self) -> String {
        format!(
            "${}: {{\n{}\n}}",
            self.id,
            self.body
                .iter()
                .filter_map(|v| v.serialize_node())
                .indented(1)
                .join("\n")
        )
    }
}
