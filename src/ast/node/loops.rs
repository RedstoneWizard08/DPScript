use super::{Node, Variable};
use crate::Spanned;
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Loop {
    /// The variable name for the loop entry
    pub var_name: Spanned<String>,

    /// The array we are looping through
    pub array: Spanned<String>,

    /// The span
    pub span: SourceSpan,

    /// The loop body
    pub body: Vec<Node>,

    /// A cache of local variables defined in the function.
    pub vars: Option<Vec<Variable>>,
}

impl Loop {
    fn cache_vars(&mut self) -> &mut Self {
        let mut vars = Vec::new();

        for node in &self.body {
            if let Node::Variable(var) = node {
                vars.push(var.clone());
            }
        }

        self.vars = Some(vars);
        self
    }

    pub fn get_locals(&mut self) -> Vec<Variable> {
        if let Some(vars) = &self.vars {
            vars.clone()
        } else {
            self.cache_vars().vars.clone().unwrap_or_default()
        }
    }
}
