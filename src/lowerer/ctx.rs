use serde::{Deserialize, Serialize};

use crate::IRNode;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoweringContext {
    pub module: String,
    pub namespace: String,

    pub inits: usize,
    pub ticks: usize,

    pub init_nodes: Vec<IRNode>,
    pub tick_nodes: Vec<IRNode>,

    pub init_names: Vec<String>,
    pub tick_names: Vec<String>,
}

impl LoweringContext {
    pub fn new(ns: impl AsRef<str>) -> Self {
        Self {
            namespace: ns.as_ref().into(),
            ..Default::default()
        }
    }
}
