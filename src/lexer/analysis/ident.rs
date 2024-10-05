use crate::{Node, ParserResult, Spanned, Token, TokenCursor};

use super::Analyzer;

impl Analyzer<Spanned<String>> for String {
    fn analyze(
        item: Spanned<Token>,
        _cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Spanned<String>>> {
        Ok(match item.0 {
            Token::Ident(id) => Some((id, item.1)),
            _ => None,
        })
    }
}
