use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{compiler::Compilable, expr::Expr, state::State, Result};

use super::nbt::{Nbt, NbtItem};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
    pub from_expr: Option<Box<Expr>>,
    pub values: HashMap<String, NbtItem>,
}

impl Component {
    pub fn new(text: impl AsRef<str>) -> Self {
        let mut map = HashMap::new();

        map.insert("text".into(), NbtItem::String(text.as_ref().into()));

        Self {
            values: map,
            from_expr: None,
        }
    }

    pub fn merge(&mut self, nbt: Nbt) {
        self.values.extend(nbt.data);
    }

    pub fn from_map(map: HashMap<String, NbtItem>) -> Self {
        Self {
            values: map,
            from_expr: None,
        }
    }

    pub fn from_expr(expr: Expr) -> Self {
        Self {
            from_expr: Some(Box::new(expr)),
            values: HashMap::new(),
        }
    }
}

impl Compilable for Component {
    fn compile(&self, state: &mut State) -> Result<String> {
        let mut me = self.clone();

        if let Some(from) = &self.from_expr {
            me.merge(Nbt {
                ty: None,
                data: from.as_component(state)?.values,
            });
        }

        let mut b = String::new();

        for (k, v) in &self.values {
            b.push_str(&format!("\"{}\": {},", k, v.compile(state)?));
        }

        Ok(format!("{{{}}}", b.trim_end_matches(',')))
    }
}
