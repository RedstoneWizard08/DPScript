use super::SerializeNode;
use crate::{IRDefinition, VariableAlias};

impl SerializeNode for IRDefinition {
    fn serialize_node(&self) -> String {
        match self {
            Self::VariableAlias(it) => it.serialize_node(),
        }
    }
}

impl SerializeNode for VariableAlias {
    fn serialize_node(&self) -> String {
        format!(
            "define variable_alias {}: \"{}\" @ \"{}\";\n",
            self.name, self.store, self.path
        )
    }
}
