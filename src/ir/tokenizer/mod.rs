mod inner;
mod token;

pub use token::*;

use crate::{Cursor, Result, Spanned};

pub type IRStringCursor = Cursor<String, String>;

#[derive(Debug, Clone)]
pub struct IRTokenizer {
    pub tokens: Vec<Spanned<IRToken>>,
    pub cursor: IRStringCursor,
}

impl IRTokenizer {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self {
            tokens: Vec::new(),
            cursor: IRStringCursor::new_from_code(data),
        }
    }

    pub fn tokenize(&mut self) -> Result<&mut Self> {
        while let Some(ch) = self.cursor.next() {
            self.tokenize_inner(ch)?;
        }

        Ok(self)
    }

    pub fn tokens(&self) -> Vec<Spanned<IRToken>> {
        self.tokens.clone()
    }
}
