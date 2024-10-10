use std::collections::HashMap;

use super::{Node, Subroutine, Variable};
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Conditional {
    /// The condition
    pub condition: Box<Node>,

    /// The body
    pub body: Vec<Node>,

    /// If there is an else condition, this is its body
    pub else_body: Vec<Node>,

    /// The span
    pub span: SourceSpan,

    /// A cache of local variables defined in the conditional if block.
    pub if_locals: Option<Vec<Variable>>,

    /// A cache of local variables defined in the conditional else block.
    pub else_locals: Option<Vec<Variable>>,
}

impl Conditional {
    fn cache_vars(&mut self) -> &mut Self {
        let mut vars = Vec::new();

        for node in &self.body {
            if let Node::Variable(var) = node {
                vars.push(var.clone());
            }
        }

        self.if_locals = Some(vars);
        let mut vars = Vec::new();

        for node in &self.else_body {
            if let Node::Variable(var) = node {
                vars.push(var.clone());
            }
        }

        self.else_locals = Some(vars);
        self
    }

    pub fn get_if_locals(&mut self) -> Vec<Variable> {
        if let Some(vars) = &self.if_locals {
            vars.clone()
        } else {
            self.cache_vars().if_locals.clone().unwrap_or_default()
        }
    }

    pub fn get_else_locals(&mut self) -> Vec<Variable> {
        if let Some(vars) = &self.else_locals {
            vars.clone()
        } else {
            self.cache_vars().else_locals.clone().unwrap_or_default()
        }
    }

    pub fn get_if_subroutines(&self) -> HashMap<String, Subroutine> {
        let mut map = HashMap::new();

        for item in &self.body {
            if let Node::Subroutine(sub) = item {
                map.insert(sub.name.clone().0, sub.clone());
            }
        }

        map
    }

    pub fn get_else_subroutines(&self) -> HashMap<String, Subroutine> {
        let mut map = HashMap::new();

        for item in &self.else_body {
            if let Node::Subroutine(sub) = item {
                map.insert(sub.name.clone().0, sub.clone());
            }
        }

        map
    }
}
