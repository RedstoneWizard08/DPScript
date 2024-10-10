mod block;
mod call;
mod cond;
mod func;
mod loops;
mod module;
mod ops;
mod ret;
mod var;

use crate::{Node, Spanned};

pub trait Remap {
    fn remap_name(&self, orig: &String, new: &String) -> Self;
}

impl Remap for Spanned<String> {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        if self.0 == *orig {
            (new.clone(), self.1)
        } else {
            self.clone()
        }
    }
}

impl Remap for Vec<Node> {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        self.iter().map(|v| v.remap_name(orig, new)).collect()
    }
}

impl Remap for Node {
    fn remap_name(&self, orig: &String, new: &String) -> Self {
        match self.clone() {
            Node::Block(it) => Node::Block(it.remap_name(orig, new)),
            Node::Call(it) => Node::Call(it.remap_name(orig, new)),
            Node::Module(it) => Node::Module(it.remap_name(orig, new)),
            Node::Function(it) => Node::Function(it.remap_name(orig, new)),
            Node::Variable(it) => Node::Variable(it.remap_name(orig, new)),
            Node::Operation(it) => Node::Operation(it.remap_name(orig, new)),
            Node::Loop(it) => Node::Loop(it.remap_name(orig, new)),
            Node::Return(it) => Node::Return(it.remap_name(orig, new)),
            Node::Conditional(it) => Node::Conditional(it.remap_name(orig, new)),
            Node::Ident(it) => Node::Ident(it.remap_name(orig, new)),

            other => other,
        }
    }
}
