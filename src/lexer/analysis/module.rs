use super::Analyzer;
use crate::{AddSpan, Module, Node, Result, Spanned, Token, TokenCursor};

impl Analyzer<Module> for Module {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Module>> {
        let mut is_pub = match item.0 {
            Token::Module => false,
            Token::Pub => true,
            _ => return Ok(None),
        };

        if is_pub {
            if !cursor.peek().is_some_and(|(v, _)| v == Token::Module) {
                return Ok(None);
            }

            item = cursor.next().unwrap();
        }

        let mut name = Vec::new();
        let mut is_block = false;

        while let Some((tkn, span)) = cursor.next() {
            if tkn == Token::Semi {
                break;
            }

            if tkn == Token::LeftBrace {
                is_block = true;
                break;
            }

            if tkn == Token::Slash {
                continue;
            }

            if let Token::Ident(id) = tkn {
                name.push((id, span));
            }
        }

        if !is_block {
            is_pub = true;
        }

        let mut buf = Vec::new();
        let mut opens = 0;

        while let Some((tkn, span)) = cursor.next() {
            if is_block {
                if tkn == Token::RightBrace {
                    if opens == 0 {
                        break;
                    } else {
                        opens -= 1;
                    }
                }

                if tkn == Token::LeftBrace {
                    opens += 1;
                }
            }

            buf.push((tkn, span));
        }

        let mut span = item.1;

        if let Some(tkn) = buf.last() {
            span = span.add(tkn.1);
        }

        let mut body = Vec::new();
        let mut buf_cursor =
            TokenCursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

        while let Some(item) = buf_cursor.next() {
            Node::analyze(item, &mut buf_cursor, &mut body)?;
        }

        Ok(Some(Self {
            is_pub,
            name,
            body,
            span,
            top_level: None,
            source: cursor.source(),
            imported_objects: None,
        }))
    }
}
