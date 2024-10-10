mod checker;
mod ctx;
mod helpers;

pub use checker::*;
pub use ctx::*;

use super::IRAst;
use crate::Result;

/// The validator for IR ASTs.
/// This also doubles as a first processing pass.
#[derive(Debug, Clone)]
pub struct IRValidator {
    pub ast: IRAst,
}

impl IRValidator {
    pub fn new(ast: IRAst) -> Self {
        Self { ast }
    }

    pub fn run(mut self) -> Result<Self> {
        self.ast.indexed = false;
        self.ast.index();

        let mut cx = self.ast.create_checker_context();

        for node in &mut self.ast.nodes {
            node.check(&mut cx)?;
        }

        Ok(self)
    }

    pub fn get_code(&self) -> String {
        self.ast.serialize_nodes()
    }
}
