use super::Analyzer;
use crate::{check_token, AddSpan, Call, Cursor, Node, ParserResult, Spanned, Token, TokenCursor};

impl Analyzer<Call> for Call {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Call>> {
        if cursor.peek().is_some_and(|(v, _)| v != Token::LeftParen) || cursor.peek().is_none() {
            return Ok(None);
        }

        let Some(function) = String::analyze(item, cursor, nodes)? else {
            return Ok(None);
        };

        let mut span = function.1.clone();

        check_token!(remove cursor == LeftParen);

        let mut args = Vec::new();

        if cursor.peek().is_some_and(|(v, _)| v != Token::RightParen) {
            let mut buf = Vec::new();
            let mut cur = Vec::new();
            let mut opens = 0;
            let mut brace_opens = 0;
            let mut bracket_opens = 0;

            while let Some((tkn, span)) = cursor.next() {
                if tkn == Token::RightParen {
                    if opens == 0 {
                        if !cur.is_empty() {
                            buf.push(cur);
                        }

                        break;
                    } else {
                        opens -= 1;
                    }
                }

                if tkn == Token::LeftParen {
                    opens += 1;
                }

                if tkn == Token::RightBracket {
                    bracket_opens -= 1;
                }

                if tkn == Token::LeftBracket {
                    bracket_opens += 1;
                }

                if tkn == Token::RightBrace {
                    brace_opens -= 1;
                }

                if tkn == Token::LeftBrace {
                    brace_opens += 1;
                }

                if tkn == Token::Comma && opens == 0 && brace_opens == 0 && bracket_opens == 0 {
                    buf.push(cur);
                    cur = Vec::new();
                    continue;
                }

                cur.push((tkn, span));
            }

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
                    Node::analyze(item, &mut buf_cursor, &mut args)?;
                }
            }
        } else {
            check_token!(remove cursor == RightParen);
        }

        Ok(Some(Self {
            function,
            args,
            span,
        }))
    }
}
