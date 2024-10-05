pub mod node;

pub use node::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AST {
    pub nodes: Vec<Node>,
}
