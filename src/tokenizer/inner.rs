use super::{StringCursor, Token};
use crate::{error::ParserError, util::IsNotIdent, ParserResult, Spanned};

pub(crate) fn tokenize_inner(
    ch: char,
    cursor: &mut StringCursor,
    tokens: &mut Vec<Spanned<Token>>,
) -> ParserResult<()> {
    if ch.is_whitespace() {
        return Ok(());
    }

    if ch == '/' {
        if cursor.peek_ahead(0).is_some_and(|v| v == '/') {
            // println!("==>>> SKIPPING COMMENT");

            while let Some(ch) = cursor.next() {
                if ch == '\n' {
                    break;
                }
            }

            return Ok(());
        }
    }

    let sym = match ch {
        ':' => Some((Token::Colon, cursor.span(1))),
        ',' => Some((Token::Comma, cursor.span(1))),
        '[' => Some((Token::LeftBracket, cursor.span(1))),
        ']' => Some((Token::RightBracket, cursor.span(1))),
        '{' => Some((Token::LeftBrace, cursor.span(1))),
        '}' => Some((Token::RightBrace, cursor.span(1))),
        '(' => Some((Token::LeftParen, cursor.span(1))),
        ')' => Some((Token::RightParen, cursor.span(1))),
        '<' => Some((Token::LeftAngle, cursor.span(1))),
        '>' => Some((Token::RightAngle, cursor.span(1))),
        ';' => Some((Token::Semi, cursor.span(1))),
        '=' => Some((Token::Equal, cursor.span(1))),
        '-' => Some((Token::Minus, cursor.span(1))),
        '+' => Some((Token::Plus, cursor.span(1))),
        '*' => Some((Token::Star, cursor.span(1))),
        '/' => Some((Token::Slash, cursor.span(1))),
        '&' => Some((Token::And, cursor.span(1))),
        '#' => Some((Token::Hash, cursor.span(1))),

        '.' => {
            if cursor.peek().is_some_and(|v| v == '.')
                && cursor.peek_ahead(1).is_some_and(|v| v == '.')
            {
                let span = cursor.span(3);
                cursor.skip(2);
                Some((Token::Ellipsis, span))
            } else if cursor.peek().is_some_and(|v| v == '.') {
                let span = cursor.span(2);
                cursor.skip(1);
                Some((Token::Range, span))
            } else {
                Some((Token::Dot, cursor.span(1)))
            }
        }

        _ => None,
    };

    if let Some(tkn) = sym {
        tokens.push(tkn);
        return Ok(());
    }

    let kw = match ch {
        'i' => {
            if let Some(next) = cursor.peek() {
                let it = match next {
                    'f' => Some((Token::If, cursor.span(2))),
                    'd' => Some((Token::Id, cursor.span(2))),

                    'n' => {
                        if cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident()) {
                            Some((Token::In, cursor.span(2)))
                        } else if cursor.peek_many(1, 2).is_some_and(|v| v == "it")
                            && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                        {
                            let span = cursor.span(4);
                            cursor.skip(2);
                            Some((Token::Init, span))
                        } else {
                            None
                        }
                    }

                    'm' => {
                        if cursor.peek_many(1, 4).is_some_and(|v| v == "port")
                            && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                        {
                            let span = cursor.span(6);
                            cursor.skip(4);
                            Some((Token::Import, span))
                        } else {
                            None
                        }
                    }

                    _ => None,
                };

                if it.is_some() {
                    cursor.skip(1);
                }

                it
            } else {
                None
            }
        }

        's' => {
            if cursor.peek_many(0, 4).is_some_and(|v| v == "tore")
                && cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(5);
                cursor.skip(4);
                Some((Token::Store, span))
            } else if cursor.peek_many(0, 7).is_some_and(|v| v == "elector")
                && cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(8);
                cursor.skip(7);
                Some((Token::Selector, span))
            } else {
                None
            }
        }

        'e' => {
            if cursor.peek_many(0, 5).is_some_and(|v| v == "xport")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(6);
                cursor.skip(5);
                Some((Token::Export, span))
            } else if cursor.peek_many(0, 3).is_some_and(|v| v == "num")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(4);
                cursor.skip(3);
                Some((Token::Enum, span))
            } else {
                None
            }
        }

        'f' => {
            if cursor.peek_many(0, 1).is_some_and(|v| v == "n")
                && cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(2);
                cursor.skip(1);
                Some((Token::Fn, span))
            } else if cursor.peek_many(0, 2).is_some_and(|v| v == "or")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(3);
                cursor.skip(2);
                Some((Token::For, span))
            } else if cursor.peek_many(0, 5).is_some_and(|v| v == "acade")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(6);
                cursor.skip(5);
                Some((Token::Facade, span))
            } else if cursor.peek_many(0, 4).is_some_and(|v| v == "alse")
                && cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(5);
                cursor.skip(4);
                Some((Token::Bool(false), span))
            } else {
                None
            }
        }

        'p' => {
            if cursor.peek_many(0, 2).is_some_and(|v| v == "ub")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(3);
                cursor.skip(2);
                Some((Token::Pub, span))
            } else if cursor.peek_many(0, 3).is_some_and(|v| v == "ath")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(4);
                cursor.skip(3);
                Some((Token::Path, span))
            } else if cursor.peek_many(0, 5).is_some_and(|v| v == "layer")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(6);
                cursor.skip(5);
                Some((Token::Player, span))
            } else {
                None
            }
        }

        'c' => {
            if cursor.peek_many(0, 4).is_some_and(|v| v == "onst")
                && cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(5);
                cursor.skip(4);
                Some((Token::Const, span))
            } else if cursor.peek_many(0, 7).is_some_and(|v| v == "ompiler")
                && cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(8);
                cursor.skip(7);
                Some((Token::Compiler, span))
            } else if cursor.peek_many(0, 8).is_some_and(|v| v == "omponent")
                && cursor.peek_ahead(8).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(9);
                cursor.skip(8);
                Some((Token::Component, span))
            } else {
                None
            }
        }

        'l' => {
            if cursor.peek_many(0, 2).is_some_and(|v| v == "et")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(3);
                cursor.skip(2);
                Some((Token::Let, span))
            } else {
                None
            }
        }

        'r' => {
            if cursor.peek_many(0, 5).is_some_and(|v| v == "eturn")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(6);
                cursor.skip(5);
                Some((Token::Return, span))
            } else {
                None
            }
        }

        'o' => {
            if cursor.peek_many(0, 8).is_some_and(|v| v == "bjective")
                && cursor.peek_ahead(8).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(9);
                cursor.skip(8);
                Some((Token::Objective, span))
            } else {
                None
            }
        }

        'm' => {
            if cursor.peek_many(0, 5).is_some_and(|v| v == "odule")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(6);
                cursor.skip(5);
                Some((Token::Module, span))
            } else {
                None
            }
        }

        'n' => {
            if cursor.peek_many(0, 2).is_some_and(|v| v == "bt")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(3);
                cursor.skip(2);
                Some((Token::Nbt, span))
            } else {
                None
            }
        }

        't' => {
            if cursor.peek_many(0, 3).is_some_and(|v| v == "ick")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(4);
                cursor.skip(3);
                Some((Token::Tick, span))
            } else if cursor.peek_many(0, 3).is_some_and(|v| v == "rue")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                let span = cursor.span(4);
                cursor.skip(3);
                Some((Token::Bool(true), span))
            } else {
                None
            }
        }

        _ => None,
    };

    if let Some(tkn) = kw {
        tokens.push(tkn);

        return Ok(());
    }

    if ch == '"' {
        let mut s = Vec::new();
        let c2 = cursor.clone();

        while let Some(tkn) = cursor.next() {
            if tkn == '"' {
                break;
            } else {
                s.push(tkn);
            }
        }

        tokens.push((Token::String(s.iter().collect()), c2.span(s.len() + 2)));

        return Ok(());
    }

    if ch.is_ascii_digit() {
        let mut buf = Vec::new();
        let c2 = cursor.clone();

        buf.push(ch);

        while let Some(tkn) = cursor.peek() {
            if tkn.is_ascii_digit() || (tkn == '.' && !buf.contains(&'.')) {
                buf.push(tkn);
                cursor.skip(1);
            } else {
                break;
            }
        }

        let span = c2.span(buf.len());

        if buf.contains(&'.') {
            if let Ok(it) = buf.iter().collect::<String>().parse() {
                tokens.push((Token::Float(it), span));
            } else {
                return Err(ParserError {
                    src: cursor.source(),
                    at: span,
                    err: format!(
                        "Could not parse as a float: {}",
                        buf.iter().collect::<String>()
                    ),
                });
            }
        } else {
            if let Ok(it) = buf.iter().collect::<String>().parse() {
                tokens.push((Token::Int(it), span));
            } else {
                return Err(ParserError {
                    src: cursor.source(),
                    at: span,
                    err: format!(
                        "Could not parse as an int: {}",
                        buf.iter().collect::<String>()
                    ),
                });
            }
        }

        return Ok(());
    }

    if ch.is_ascii_alphabetic() {
        let mut ident = Vec::new();
        let c2 = cursor.clone();

        ident.push(ch);

        while let Some(tkn) = cursor.peek() {
            if tkn.is_ascii_alphanumeric() || tkn == '_' {
                ident.push(tkn);
                cursor.skip(1);
            } else {
                break;
            }
        }

        tokens.push((Token::Ident(ident.iter().collect()), c2.span(ident.len())));

        return Ok(());
    }

    Err(ParserError {
        src: cursor.source(),
        at: cursor.span(1),
        err: format!("Unexpected character during tokenization: {}", ch),
    })
}
