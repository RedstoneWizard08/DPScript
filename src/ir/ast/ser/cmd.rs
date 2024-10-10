use super::SerializeNode;
use crate::IRCommand;

impl SerializeNode for IRCommand {
    fn serialize_node(&self) -> String {
        format!(
            "command: {};",
            self.cmd
                .iter()
                .filter_map(|v| v.serialize_node())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
