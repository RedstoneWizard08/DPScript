use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRTag {
    /// The namespaced ID of the tag, including its prefix.
    /// Ex: A function tag for init would be `minecraft:tags/functions/load` instead of `minecraft:load`.
    pub name: String,

    /// The tag entries.
    pub entries: Vec<String>,
}
