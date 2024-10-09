use crate::{IRBlock, IRDefinition, IRNode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoweringContext {
    pub module: String,
    pub namespace: String,

    pub blocks: usize,
    pub inits: usize,
    pub ticks: usize,

    pub init_nodes: Vec<IRNode>,
    pub tick_nodes: Vec<IRNode>,

    pub init_names: Vec<String>,
    pub tick_names: Vec<String>,

    pub defs: BTreeMap<String, IRDefinition>,
    pub block_nodes: Vec<IRBlock>,
    pub extra_nodes: Vec<IRNode>,

    pub join_dirty: bool,
    pub join_block: Option<IRBlock>,
}

impl LoweringContext {
    pub fn new(ns: impl AsRef<str>) -> Self {
        Self {
            namespace: ns.as_ref().into(),
            ..Default::default()
        }
    }
}
