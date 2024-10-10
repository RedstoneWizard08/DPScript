use super::{IRBlock, IRDefinition, IRNode, VariableAlias};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRFunction {
    /// The namespaced ID of this function.
    pub id: String,

    /// The body of the function.
    pub body: Vec<IRNode>,
}

impl IRFunction {
    pub fn get_locals(&self) -> HashMap<String, VariableAlias> {
        let mut map = HashMap::new();

        for item in &self.body {
            if let IRNode::Definition(IRDefinition::VariableAlias(it)) = item {
                map.insert(it.name.clone(), it.clone());
            }
        }

        map
    }

    pub fn get_blocks(&self) -> HashMap<String, IRBlock> {
        let mut map = HashMap::new();

        for item in &self.body {
            if let IRNode::Block(it) = item {
                map.insert(it.id.clone(), it.clone());
            }
        }

        map
    }
}
