pub mod attr;
pub mod block;
pub mod call;
pub mod func;
pub mod ident;
pub mod import;
pub mod literal;
pub mod loops;
pub mod module;
pub mod node;
pub mod ops;
pub mod ty;
pub mod var;

use super::TokenCursor;
use crate::{Node, ParserResult, Spanned, Token};

pub trait Analyzer<T> {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<T>>;
}
