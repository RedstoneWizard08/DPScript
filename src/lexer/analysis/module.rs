use crate::{
    check_token, AddSpan, Cursor, Module, Node, ParserError, ParserResult, Spanned, Token,
    TokenCursor,
};

use super::Analyzer;

impl Analyzer<Module> for Module {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Module>> {
        if item.0 != Token::Module {
            return Ok(None);
        }

        debug!("This is a module node, trying to parse...");

        let (name, name_span) = cursor.next_or_die(item.1)?;

        debug!("Checking name...");

        let name = match name {
            Token::Ident(id) => (id, name_span),

            _ => {
                return Err(ParserError {
                    src: cursor.source(),
                    at: name_span,
                    err: format!("Unexpected token while parsing a module: {}", name),
                })
            }
        };

        debug!("Checking semi...");

        check_token!(remove cursor == Semi);

        debug!("Building a buffer...");

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
