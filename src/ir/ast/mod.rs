mod node;
mod ser;

pub use node::*;
pub use ser::*;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct IRAst {
    pub nodes: Vec<IRNode>,
}

impl IRAst {
    pub fn serialize_nodes(&self) -> String {
        let mut parts = Vec::new();

        parts.push(
            IRNode::Definition(IRDefinition::VariableAlias(VariableAlias {
                name: "__RETURN_VAL__".into(),
                store: "dpscript:core/funcs".into(),
                path: "return_value".into(),
            }))
            .serialize_node(),
        );

        for node in &self.nodes {
            parts.push(node.serialize_node());
        }

        parts.join("\n")
    }

    pub fn merge(&mut self, other: IRAst) -> &mut Self {
        self.nodes.extend(other.nodes);
        self
    }
}
