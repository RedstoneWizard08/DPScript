pub mod node;

pub use node::*;

#[derive(Debug, Clone)]
pub struct AST {
    pub nodes: Vec<Node>,
}
