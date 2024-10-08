use super::SerializeNode;
use crate::IRCommand;

impl SerializeNode for IRCommand {
    fn serialize_node(&self) -> String {
        format!("command: {};", self.cmd.serialize_node())
    }
}
