use std::collections::HashMap;

use crate::nbt::NbtItem;

#[derive(Debug, Clone, PartialEq)]
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
