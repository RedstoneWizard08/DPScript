use super::SerializeNode;
use crate::IRExecute;

impl SerializeNode for IRExecute {
    fn serialize_node(&self) -> String {
        format!("execute: \"{}\", ${};", self.selector, self.block)
    }
}
