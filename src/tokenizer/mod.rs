pub(crate) mod inner;
pub mod token;

pub use token::*;

use crate::{util::Cursor, Result, Spanned};
use inner::tokenize_inner;
use miette::NamedSource;

pub type StringCursor = Cursor<String, NamedSource<String>>;

pub fn tokenize(file: impl AsRef<str>, data: impl AsRef<str>) -> Result<Vec<Spanned<Token>>> {
    let mut tokens = Vec::new();
    let mut cursor = StringCursor::new_from_code(file, data);

    while let Some(ch) = cursor.next() {
        tokenize_inner(ch, &mut cursor, &mut tokens)?;
    }

    Ok(tokens)
}
