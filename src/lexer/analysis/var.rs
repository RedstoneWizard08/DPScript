use crate::{
    check_token, Cursor, Node, ParserError, ParserResult, Spanned, Token, TokenCursor, Type,
    Variable,
};

use super::Analyzer;

impl Analyzer<Variable> for Variable {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Variable>> {
        let is_const = match item.0 {
            Token::Let => false,
            Token::Const => true,
            _ => return Ok(None),
        };

        let (name, name_span) = cursor.next_or_die(item.1)?;

        let name = match name {
            Token::Ident(id) => (id, name_span),

            _ => {
                return Err(ParserError {
                    src: cursor.source(),
                    at: name_span,
                    err: format!("Unexpected token while parsing a variable: {}", name),
                })
            }
        };

        let mut ty = None;

        if cursor.peek().is_some_and(|(v, _)| v == Token::Colon) {
            let (_, sp) = cursor.next().unwrap();
            let it = cursor.next_or_die(sp)?;

            ty = Type::analyze(it, cursor, nodes)?;
        }

        let it = check_token!(remove cursor == Equal).unwrap();

        let mut buf = Vec::new();

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::Semi {
                break;
            }

            buf.push((tkn, span));
        }

        let mut nodes = Vec::new();

        let mut buf_cursor =
            Cursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut nodes)?;
        }

        // We use this method to allow `Operation` nodes to work
        let value = nodes.first();

        match value {
            Some(value) => Ok(Some(Self {
                is_const,
                name,
                ty,
                value: Box::new(value.clone()),
            })),

            None => Err(ParserError {
                src: cursor.source(),
                at: it.1,
                err: format!("Could not parse a node: {}", it.0),
            }),
        }
    }
}