use std::collections::HashMap;

use crate::{expr::Expr, nbt::{Nbt, NbtItem}};

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub from_expr: Option<Box<Expr>>,
    pub values: HashMap<String, NbtItem>,
}

impl Component {
    pub fn new(text: impl AsRef<str>) -> Self {
        let mut map = HashMap::new();

        map.insert("text".into(), NbtItem::String(text.as_ref().into()));

        Self { values: map, from_expr: None }
    }

    pub fn merge(&mut self, nbt: Nbt) {
        self.values.extend(nbt.data);
    }

    pub fn from_expr(expr: Expr) -> Self {
        Self {
            from_expr: Some(Box::new(expr)),
            values: HashMap::new(),
        }
    }
}
