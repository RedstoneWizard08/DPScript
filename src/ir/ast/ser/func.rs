use super::SerializeNode;
use crate::{IRFunction, Indented};

impl SerializeNode for IRFunction {
    fn serialize_node(&self) -> String {
        format!(
            "func \"{}\": {{\n{}\n}}",
            self.id,
            self.body
                .iter()
                .map(|v| v.serialize_node())
                .indented(1)
                .join("\n")
        )
    }
}
