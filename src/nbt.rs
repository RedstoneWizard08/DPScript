use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    compiler::Compilable, error::CompilationError, source, state::State, Result, DPSCRIPT_VAR_STORE,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nbt {
    pub ty: Option<String>,
    pub data: HashMap<String, NbtItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NbtItem {
    String(String),
    Ident(String),
    Int(i32),
    Float(f32),
    Map(Box<HashMap<String, NbtItem>>),
    Array(Box<Vec<NbtItem>>),
    Bool(bool),
}

impl Compilable for Nbt {
    fn compile(&self, state: &mut State) -> Result<String> {
        let mut b = String::new();

        for (k, v) in &self.data {
            b.push_str(&format!("\"{}\": {},", k, v.compile(state)?));
        }

        Ok(format!("{{{}}}", b.trim_end_matches(',')))
    }
}

impl Compilable for NbtItem {
    fn compile(&self, state: &mut State) -> Result<String> {
        match self.clone() {
            Self::String(s) => Ok(format!("\"{}\"", s)),

            Self::Ident(s) => {
                if let Some(var) = state.locals.get(&s) {
                    Ok(serde_json::to_string(&json!({
                        "storage": DPSCRIPT_VAR_STORE,
                        "nbt": var.0,
                        "interpret": true
                    }))
                    .unwrap())
                } else if let Some(var) = state.globals.get(&s) {
                    Ok(serde_json::to_string(&json!({
                        "storage": DPSCRIPT_VAR_STORE,
                        "nbt": var.0,
                        "interpret": true
                    }))
                    .unwrap())
                } else {
                    Err(CompilationError {
                        src: source!(state),
                        err: format!("Cannot find a variable named {}!", s),
                    })
                }
            }

            Self::Int(i) => Ok(i.to_string()),
            Self::Float(f) => Ok(f.to_string()),
            Self::Bool(b) => Ok(b.to_string()),

            Self::Map(m) => {
                let mut b = String::new();

                for (k, v) in &*m {
                    b.push_str(&format!("\"{}\": {},", k, v.compile(state)?));
                }

                Ok(format!("{{{}}}", b.trim_end_matches(',')))
            }

            Self::Array(arr) => Ok(format!(
                "[{}]",
                arr.iter()
                    .map(|v| v.compile(state).unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
        }
    }
}
