use super::SerializeNode;
use crate::{IRTag, Indented};

impl SerializeNode for IRTag {
    fn serialize_node(&self) -> String {
        format!(
            "tag \"{}\": [\n{}\n];",
            self.name,
            self.entries
                .iter()
                .map(|v| format!("\"{}\";", v))
                .indented(1)
                .join("\n")
        )
    }
}
