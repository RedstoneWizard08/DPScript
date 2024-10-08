use crate::IRCall;

use super::SerializeNode;

impl SerializeNode for IRCall {
    fn serialize_node(&self) -> String {
        format!("call: \"{}\";", self.function)
    }
}
