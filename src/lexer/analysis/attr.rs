use crate::{
    check_token, AddSpan, Attribute, AttributeValue, ParserError, ParserResult, Spanned, Token,
    TokenCursor,
};

use super::Analyzer;

impl Analyzer<Attribute> for Attribute {
    fn analyze(item: Spanned<Token>, cursor: &mut TokenCursor) -> ParserResult<Option<Attribute>> {
        if item.0 == Token::Hash && cursor.peek().is_some_and(|(v, _)| v == Token::LeftBracket) {
            cursor.skip(1);

            let mut buf = Vec::new();
            let mut opens = 0;

            while let Some((token, span)) = cursor.next() {
                if token == Token::RightBracket {
                    if opens == 0 {
                        break;
                    } else {
                        opens -= 1;
                    }
                }

                buf.push((token, span));
            }

            let (name, name_span) = buf.remove(0);

            let name = match name {
                Token::Ident(id) => (id, name_span),

                _ => {
                    return Err(ParserError {
                        src: cursor.source(),
                        at: name_span,
                        err: format!("Unexpected token: {}", name),
                    });
                }
            };

            check_token!(remove cursor => buf[0] == LeftParen);

            let (value, value_span) = buf.remove(0);

            let value = match value {
                Token::Ident(id) => AttributeValue::Ident((id, value_span)),
                Token::String(s) => AttributeValue::String((s, value_span)),

                _ => {
                    return Err(ParserError {
                        src: cursor.source(),
                        at: value_span,
                        err: format!("Unexpected token: {}", value),
                    });
                }
            };

            check_token!(cursor => buf[0] == RightParen);

            return Ok(Some(Self {
                name,
                value,
                span: item.1.add(buf.get(0).unwrap().1),
            }));
        }

        Ok(None)
    }
}
