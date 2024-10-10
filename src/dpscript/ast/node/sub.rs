use std::collections::HashMap;

use super::{Node, Variable};
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Subroutine {
    /// The name of the subroutine
    pub name: Spanned<String>,

    /// The span
    pub span: SourceSpan,

    /// The subroutine body
    pub body: Vec<Node>,

    /// A cache of local variables defined in the subroutine.
    pub locals: Option<Vec<Variable>>,
}

impl Subroutine {
    fn cache_vars(&mut self) -> &mut Self {
        let mut vars = Vec::new();

        for node in &self.body {
            if let Node::Variable(var) = node {
                vars.push(var.clone());
            }
        }

        self.locals = Some(vars);
        self
    }

    pub fn get_locals(&mut self) -> Vec<Variable> {
        if let Some(vars) = &self.locals {
            vars.clone()
        } else {
            self.cache_vars().locals.clone().unwrap_or_default()
        }
    }

    pub fn get_subroutines(&self) -> HashMap<String, Subroutine> {
        let mut map = HashMap::new();

        for item in &self.body {
            if let Node::Subroutine(sub) = item {
                map.insert(sub.name.clone().0, sub.clone());
            }
        }

        map
    }
}
