use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagData {
    pub replace: bool,
    pub values: Vec<String>,
}
