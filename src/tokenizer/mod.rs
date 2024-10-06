pub mod inner;
pub mod token;

pub use token::*;

use crate::{util::Cursor, Result, Spanned};
use miette::NamedSource;

pub type StringCursor = Cursor<String, NamedSource<String>>;

#[derive(Debug, Clone)]
pub struct Tokenizer {
    pub tokens: Vec<Spanned<Token>>,
    pub cursor: StringCursor,
}

impl Tokenizer {
    pub fn new(file: impl AsRef<str>, data: impl AsRef<str>) -> Self {
        Self {
            tokens: Vec::new(),
            cursor: Cursor::new_from_code(file, data),
        }
    }

    pub fn tokenize(&mut self) -> Result<&mut Self> {
        while let Some(ch) = self.cursor.next() {
            self.tokenize_inner(ch)?;
        }

        Ok(self)
    }

    pub fn tokens(&self) -> Vec<Spanned<Token>> {
        self.tokens.clone()
    }
}
