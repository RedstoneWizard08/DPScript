use super::IRNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IRDataOperation {
    Set(AddDataOperation),
    Append(AddDataOperation),
    Copy(CopyDataOperation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDataOperation {
    pub var: String,
    pub value: Box<IRNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyDataOperation {
    pub source: String,
    pub target: String,
}
