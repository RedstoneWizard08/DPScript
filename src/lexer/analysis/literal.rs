use crate::{Literal, ParserResult, Spanned, Token, TokenCursor};

use super::Analyzer;

impl Analyzer<Literal> for Literal {
    fn analyze(item: Spanned<Token>, _cursor: &mut TokenCursor) -> ParserResult<Option<Literal>> {
        Ok(match item.0 {
            Token::Int(i) => Some(Literal::Int((i, item.1))),
            Token::Float(f) => Some(Literal::Float((f, item.1))),
            Token::Bool(b) => Some(Literal::Bool((b, item.1))),
            Token::String(s) => Some(Literal::String((s, item.1))),
            _ => None,
        })
    }
}
