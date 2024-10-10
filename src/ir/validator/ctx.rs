use crate::{IRBlock, IRFunction, IRTag, VariableAlias};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize)]
pub struct IRCheckerContext {
    pub funcs: BTreeMap<String, IRFunction>,
    pub vars: BTreeMap<String, VariableAlias>,
    pub tags: BTreeMap<String, IRTag>,

    // State
    pub cur_fn: Option<IRFunction>,
    pub cur_block: Option<IRBlock>,
}

impl IRCheckerContext {
    pub fn get_blocks(&self) -> HashMap<String, IRBlock> {
        if let Some(func) = &self.cur_fn {
            func.get_blocks()
        } else {
            HashMap::new()
        }
    }

    pub fn get_refs(&self) -> HashMap<String, VariableAlias> {
        let mut refs: HashMap<String, VariableAlias> = HashMap::new();

        refs.extend(self.vars.clone());

        if let Some(it) = &self.cur_fn {
            refs.extend(it.get_locals());
        }

        if let Some(it) = &self.cur_block {
            refs.extend(it.get_locals());
        }

        refs
    }
}
