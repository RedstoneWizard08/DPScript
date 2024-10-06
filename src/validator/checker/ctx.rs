use crate::{Block, Enum, Export, Function, Import, Module, Objective, Variable};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct CheckerContext {
    pub modules: HashMap<String, Module>,
    pub imports: HashMap<String, Vec<Import>>,
    pub funcs: HashMap<String, Vec<Function>>,
    pub vars: HashMap<String, Vec<Variable>>,
    pub blocks: HashMap<String, Vec<Block>>,
    pub enums: HashMap<String, Vec<Enum>>,
    pub objectives: HashMap<String, Vec<Objective>>,
    pub exports: HashMap<String, Vec<Export>>,
}
