use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum NbtItem {
    String(String),
    Ident(String),
    Int(i32),
    Float(f32),
    Map(Box<HashMap<String, NbtItem>>),
    Array(Box<Vec<NbtItem>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nbt {
    pub ty: Option<String>,
    pub data: HashMap<String, NbtItem>,
}
