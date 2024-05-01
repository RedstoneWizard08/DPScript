use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{compiler::Compilable, state::State, Result};

use super::nbt::NbtItem;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Selector {
    pub entity: String,
    pub params: HashMap<String, NbtItem>,
}

impl Selector {
    pub fn new(entity: impl AsRef<str>) -> Self {
        Self {
            entity: entity.as_ref().into(),
            params: HashMap::new(),
        }
    }
}

impl Compilable for Selector {
    fn compile(&self, state: &mut State) -> Result<String> {
        let mut params = String::new();

        for (k, v) in &self.params {
            params.push_str(&format!("{}={},", k, v.compile(state)?));
        }

        Ok(format!("{}[{}]", self.entity, params.trim_end_matches(',')))
    }
}
