use super::Analyzer;
use crate::{AddSpan, Cursor, Module, Node, ParserResult, Spanned, Token, TokenCursor};

impl Analyzer<Module> for Module {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Module>> {
        if item.0 != Token::Module {
            return Ok(None);
        }

        let mut name = Vec::new();

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::Semi {
                break;
            }

            if tkn == Token::Slash {
                continue;
            }

            if let Token::Ident(id) = tkn {
                name.push((id, span));
            }
        }

        let mut buf = Vec::new();

        while let Some(tkn) = cursor.next() {
            // We want ALL of the tokens.
            buf.push(tkn);
        }

        let mut span = item.1;

        if let Some(tkn) = buf.last() {
            span = span.add(tkn.1);
        }

        let mut body = Vec::new();
        let mut buf_cursor =
            Cursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut body)?;
        }

        Ok(Some(Self { name, body, span }))
    }
}
