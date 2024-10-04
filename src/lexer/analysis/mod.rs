pub mod attr;
pub mod literal;

use super::TokenCursor;
use crate::{ParserResult, Spanned, Token};

pub trait Analyzer<T> {
    fn analyze(item: Spanned<Token>, cursor: &mut TokenCursor) -> ParserResult<Option<T>>;
}
