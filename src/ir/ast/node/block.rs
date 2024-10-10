use super::{IRDefinition, IRNode, VariableAlias};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRBlock {
    /// The block ID.
    /// Ex: `$block0: { ... }` would be `block0`.
    pub id: String,

    /// The block's body.
    pub body: Vec<IRNode>,
}

impl IRBlock {
    pub fn get_locals(&self) -> HashMap<String, VariableAlias> {
        let mut map = HashMap::new();

        for item in &self.body {
            if let IRNode::Definition(IRDefinition::VariableAlias(it)) = item {
                map.insert(it.name.clone(), it.clone());
            }
        }

        map
    }
}
