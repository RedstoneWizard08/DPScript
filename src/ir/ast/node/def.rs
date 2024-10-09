use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum IRDefinition {
    // Right now, this is the only kind of definition that we need.
    VariableAlias(VariableAlias),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct VariableAlias {
    /// The name of the alias.
    pub name: String,

    /// The name of the store.
    pub store: String,

    /// The path to the value.
    pub path: String,
}
