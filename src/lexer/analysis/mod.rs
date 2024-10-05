pub mod attr;
pub mod block;
pub mod call;
pub mod cond;
pub mod enums;
pub mod export;
pub mod func;
pub mod ident;
pub mod import;
pub mod literal;
pub mod loops;
pub mod module;
pub mod node;
pub mod objective;
pub mod ops;
pub mod ret;
pub mod ty;
pub mod var;

use super::TokenCursor;
use crate::{Node, Result, Spanned, Token};

pub trait Analyzer<T> {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<T>>;
}
