mod analysis;

use analysis::Analyzer;

use super::{IRAst, IRNode, IRToken};
use crate::{Cursor, Result, Spanned};

pub type IRTokenCursor = Cursor<Vec<Spanned<IRToken>>, String>;

#[derive(Debug, Clone)]
pub struct IRLexer {
    pub source: String,
    pub tokens: Vec<Spanned<IRToken>>,

    nodes: Vec<IRNode>,
}

impl IRLexer {
    pub fn new(source: impl AsRef<str>, tokens: Vec<Spanned<IRToken>>) -> Self {
        IRLexer {
            source: source.as_ref().into(),
            tokens,
            nodes: Vec::new(),
        }
    }

    pub fn run(mut self) -> Result<Self> {
        let mut cursor = IRTokenCursor::new_from_src(self.source.clone(), self.tokens.clone());

        while let Some(item) = cursor.next() {
            IRNode::analyze(item, &mut cursor, &mut self.nodes)?;
        }

        Ok(self)
    }

    pub fn ast(self) -> IRAst {
        IRAst { nodes: self.nodes }
    }
}
