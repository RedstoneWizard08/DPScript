use super::Analyzer;
use crate::{AddSpan, Node, Result, Return, Spanned, Token, TokenCursor};

impl Analyzer<Return> for Return {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Return>> {
        if item.0 != Token::Return {
            return Ok(None);
        }

        let mut buf = Vec::new();
        let mut span = item.1;

        while let Some((tkn, tkn_span)) = cursor.next() {
            if tkn == Token::Semi {
                span = span.add(tkn_span);
                break;
            }

            buf.push((tkn, tkn_span));
        }

        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        let value = Node::analyze(
            buf_cursor.next_or_die(span)?,
            &mut buf_cursor,
            &mut Vec::new(),
        )?;

        Ok(Some(Self {
            span,
            value: value.map(Box::new),
        }))
    }
}
