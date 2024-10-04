pub mod analysis;

use crate::{Cursor, Node, Spanned, Token, AST};
use miette::NamedSource;

pub type TokenCursor = Cursor<Vec<Spanned<Token>>, NamedSource<String>>;

#[derive(Debug, Clone)]
pub struct Lexer {
    pub file: String,
    pub source: NamedSource<String>,
    pub tokens: Vec<Token>,

    nodes: Vec<Node>,
}

impl Lexer {
    pub fn new(file: impl AsRef<str>, source: impl AsRef<str>, tokens: Vec<Token>) -> Self {
        Lexer {
            file: file.as_ref().into(),
            source: NamedSource::new(file, source.as_ref().into()),
            tokens,
            nodes: Vec::new(),
        }
    }

    pub fn get_ast(self) -> AST {
        AST { nodes: self.nodes }
    }
}
