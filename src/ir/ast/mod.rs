mod node;
mod ser;

pub use node::*;
pub use ser::*;

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct IRAst {
    pub nodes: Vec<IRNode>,
    pub indexed: bool,

    /// A list of functions defined in the IR.
    #[serde(skip)]
    pub funcs: Option<BTreeMap<String, IRFunction>>,

    /// A list of top-level variable aliases.
    #[serde(skip)]
    pub vars: Option<BTreeMap<String, VariableAlias>>,

    /// A list of tags defined in the IR.
    #[serde(skip)]
    pub tags: Option<BTreeMap<String, IRTag>>,
}

impl IRAst {
    pub fn serialize_nodes(&self) -> String {
        let mut parts = Vec::new();

        for node in &self.nodes {
            if let Some(it) = node.serialize_node() {
                parts.push(it);
            }
        }

        parts.join("\n\n")
    }

    pub fn merge(&mut self, other: IRAst) -> &mut Self {
        self.nodes.extend(other.nodes);
        self
    }

    pub fn index(&mut self) -> &mut Self {
        if self.indexed {
            return self;
        }

        let mut funcs: BTreeMap<String, IRFunction> = BTreeMap::new();
        let mut vars: BTreeMap<String, VariableAlias> = BTreeMap::new();
        let mut tags: BTreeMap<String, IRTag> = BTreeMap::new();

        for node in &self.nodes {
            match node {
                IRNode::Function(it) => {
                    funcs.insert(it.id.clone(), it.clone());
                }

                IRNode::Definition(it) => {
                    match it {
                        IRDefinition::VariableAlias(it) => vars.insert(it.name.clone(), it.clone()),
                    };
                }

                IRNode::Tag(it) => {
                    tags.insert(it.name.clone(), it.clone());
                }

                _ => {}
            }
        }

        self.funcs = Some(funcs);
        self.vars = Some(vars);
        self.tags = Some(tags);
        self.indexed = true;

        self
    }
}
