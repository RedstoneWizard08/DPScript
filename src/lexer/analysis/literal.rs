use crate::{
    check_token, AddSpan, Cursor, Literal, Node, ParserError, ParserResult, Spanned, Token,
    TokenCursor,
};

use super::Analyzer;

impl Analyzer<Literal> for Literal {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Literal>> {
        Ok(match item.0 {
            Token::Int(i) => Some(Literal::Int((i, item.1))),
            Token::Float(f) => Some(Literal::Float((f, item.1))),
            Token::Bool(b) => Some(Literal::Bool((b, item.1))),
            Token::String(s) => Some(Literal::String((s, item.1))),

            Token::Component => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Component((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(ParserError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            })
                        }
                    },
                    span,
                )))
            }

            Token::Id => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Identifier((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(ParserError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            })
                        }
                    },
                    span,
                )))
            }

            Token::Path => {
                let (_, s) = check_token!(remove cursor == Colon).unwrap();
                let (tkn, span) = cursor.next_or_die(s)?;

                Some(Literal::Path((
                    match tkn {
                        Token::String(s) => s,

                        _ => {
                            return Err(ParserError {
                                src: cursor.source(),
                                at: span,
                                err: format!("Expected string, got: {}", tkn),
                            })
                        }
                    },
                    span,
                )))
            }

            Token::Nbt => {
                check_token!(remove cursor == Colon);
                check_token!(remove cursor == LeftBrace);

                let mut buf = Vec::new();
                let mut opens = 0;

                while let Some((tkn, span)) = cursor.next() {
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

                    buf.push((tkn, span));
                }

                let mut span = item.1;

                if let Some(tkn) = buf.last() {
                    span = span.add(tkn.1);
                }

                Some(Literal::Nbt((buf, span)))
            }

            Token::LeftBracket => {
                let mut buf = Vec::new();
                let mut cur = Vec::new();
                let mut opens = 0;
                let mut brace_opens = 0;
                let mut paren_opens = 0;

                while let Some((tkn, span)) = cursor.next() {
                    if tkn == Token::RightBracket {
                        if opens == 0 {
                            if !cur.is_empty() {
                                buf.push(cur);
                            }

                            break;
                        } else {
                            opens -= 1;
                        }
                    }

                    if tkn == Token::LeftBracket {
                        opens += 1;
                    }

                    if tkn == Token::RightBrace {
                        brace_opens -= 1;
                    }

                    if tkn == Token::LeftBrace {
                        brace_opens += 1;
                    }

                    if tkn == Token::RightParen {
                        paren_opens -= 1;
                    }

                    if tkn == Token::LeftParen {
                        paren_opens += 1;
                    }

                    if tkn == Token::Comma && opens == 0 && brace_opens == 0 && paren_opens == 0 {
                        buf.push(cur);
                        cur = Vec::new();
                        continue;
                    }

                    cur.push((tkn, span));
                }

                let mut span = item.1;
                let mut items = Vec::new();

                if let Some(tkns) = buf.last() {
                    if let Some(tkn) = tkns.last() {
                        span = span.add(tkn.1);
                    }
                }

                for buf in buf {
                    let mut buf_cursor = Cursor::new_from_src(
                        cursor.source().name(),
                        cursor.source().inner().clone(),
                        buf,
                    );

                    while let Some(item) = buf_cursor.next() {
                        Node::analyze(item, &mut buf_cursor, &mut items)?;
                    }
                }

                Some(Literal::Array((items, span)))
            }

            _ => None,
        })
    }
}
