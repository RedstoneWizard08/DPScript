mod node;

use super::IRAst;
use crate::Result;

/// The finalizer removes all shortcuts in the IR and
/// replaces them with actual commands.
#[derive(Debug, Clone)]
pub struct IRFinalizer {
    pub ast: IRAst,
}

impl IRFinalizer {
    pub fn new(ast: IRAst) -> Self {
        Self { ast }
    }

    pub fn run(mut self) -> Result<Self> {
        self.ast.indexed = false;
        self.ast.index();

        let mut cx = self.ast.create_checker_context();
        let mut extra = Vec::new();

        for node in &mut self.ast.nodes {
            node.finalize(&mut cx, &mut extra)?;
        }

        self.ast.nodes.extend(extra);

        Ok(self)
    }

    pub fn get_code(&self) -> String {
        self.ast.serialize_nodes()
    }
}
