use super::{Node, Variable};
use miette::SourceSpan;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Conditional {
    /// The condition
    pub condition: Vec<Node>,

    /// The body
    pub body: Vec<Node>,

    /// If there is an else condition, this is its body
    pub else_body: Vec<Node>,

    /// The span
    pub span: SourceSpan,

    /// A cache of local variables defined in the function.
    pub vars: Option<Vec<Variable>>,
}

impl Conditional {
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
