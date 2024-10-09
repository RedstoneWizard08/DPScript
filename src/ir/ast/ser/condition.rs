use super::SerializeNode;
use crate::IRCondition;

impl SerializeNode for IRCondition {
    fn serialize_node(&self) -> String {
        format!(
            "condition: '{}';\nif: ${};\nelse: ${};\njoin: ${};",
            self.score, self.if_block, self.else_block, self.join_block
        )
    }
}
