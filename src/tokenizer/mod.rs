use token::Token;

use crate::{
    error::TokenizerError,
    util::{Cursor, IsNotIdent},
};

pub mod token;

fn tokenize_inner(
    ch: char,
    cursor: &mut Cursor,
    tokens: &mut Vec<Token>,
) -> Result<(), TokenizerError> {
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
        ':' => Some(Token::Colon),
        ',' => Some(Token::Comma),
        '[' => Some(Token::LeftBracket),
        ']' => Some(Token::RightBracket),
        '{' => Some(Token::LeftBrace),
        '}' => Some(Token::RightBrace),
        '(' => Some(Token::LeftParen),
        ')' => Some(Token::RightParen),
        '<' => Some(Token::LeftAngle),
        '>' => Some(Token::RightAngle),
        ';' => Some(Token::Semi),
        '=' => Some(Token::Equal),
        '-' => Some(Token::Minus),
        '+' => Some(Token::Plus),
        '*' => Some(Token::Star),
        '/' => Some(Token::Slash),
        '&' => Some(Token::And),
        '#' => Some(Token::Hash),

        '.' => {
            if cursor.peek().is_some_and(|v| v == '.')
                && cursor.peek_ahead(1).is_some_and(|v| v == '.')
            {
                cursor.skip(2);
                Some(Token::Ellipsis)
            } else {
                Some(Token::Dot)
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
                    'f' => Some(Token::If),
                    'd' => Some(Token::Id),

                    'n' => {
                        if cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident()) {
                            Some(Token::In)
                        } else if cursor.peek_str(1, 2).is_some_and(|v| v == "it")
                            && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                        {
                            cursor.skip(2);
                            Some(Token::Init)
                        } else {
                            None
                        }
                    }

                    'm' => {
                        if cursor.peek_str(1, 4).is_some_and(|v| v == "port")
                            && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                        {
                            cursor.skip(4);
                            Some(Token::Import)
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
            if cursor.peek_str(0, 4).is_some_and(|v| v == "tore")
                && cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(4);
                Some(Token::Store)
            } else if cursor.peek_str(0, 7).is_some_and(|v| v == "elector")
                && cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(7);
                Some(Token::Selector)
            } else {
                None
            }
        }

        'e' => {
            if cursor.peek_str(0, 5).is_some_and(|v| v == "xport")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(5);
                Some(Token::Export)
            } else if cursor.peek_str(0, 3).is_some_and(|v| v == "num")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(3);
                Some(Token::Enum)
            } else {
                None
            }
        }

        'f' => {
            if cursor.peek_str(0, 1).is_some_and(|v| v == "n")
                && cursor.peek_ahead(1).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(1);
                Some(Token::Fn)
            } else if cursor.peek_str(0, 2).is_some_and(|v| v == "or")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(2);
                Some(Token::For)
            } else if cursor.peek_str(0, 5).is_some_and(|v| v == "acade")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(5);
                Some(Token::Facade)
            } else if cursor.peek_str(0, 4).is_some_and(|v| v == "alse")
                && cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(4);
                Some(Token::Bool(false))
            } else {
                None
            }
        }

        'p' => {
            if cursor.peek_str(0, 2).is_some_and(|v| v == "ub")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(2);
                Some(Token::Pub)
            } else if cursor.peek_str(0, 3).is_some_and(|v| v == "ath")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(3);
                Some(Token::Path)
            } else if cursor.peek_str(0, 5).is_some_and(|v| v == "layer")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(5);
                Some(Token::Player)
            } else {
                None
            }
        }

        'c' => {
            if cursor.peek_str(0, 4).is_some_and(|v| v == "onst")
                && cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(4);
                Some(Token::Const)
            } else if cursor.peek_str(0, 7).is_some_and(|v| v == "ompiler")
                && cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(7);
                Some(Token::Compiler)
            } else if cursor.peek_str(0, 8).is_some_and(|v| v == "omponent")
                && cursor.peek_ahead(8).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(8);
                Some(Token::Component)
            } else {
                None
            }
        }

        'l' => {
            if cursor.peek_str(0, 2).is_some_and(|v| v == "et")
                && cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(2);
                Some(Token::Let)
            } else {
                None
            }
        }

        'r' => {
            if cursor.peek_str(0, 5).is_some_and(|v| v == "eturn")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(5);
                Some(Token::Return)
            } else {
                None
            }
        }

        'o' => {
            if cursor.peek_str(0, 8).is_some_and(|v| v == "bjective")
                && cursor.peek_ahead(8).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(8);
                Some(Token::Objective)
            } else {
                None
            }
        }

        'm' => {
            if cursor.peek_str(0, 5).is_some_and(|v| v == "odule")
                && cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(5);
                Some(Token::Module)
            } else {
                None
            }
        }

        't' => {
            if cursor.peek_str(0, 3).is_some_and(|v| v == "ick")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(3);
                Some(Token::Tick)
            } else if cursor.peek_str(0, 3).is_some_and(|v| v == "rue")
                && cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
            {
                cursor.skip(3);
                Some(Token::Bool(true))
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

        while let Some(tkn) = cursor.next() {
            if tkn == '"' {
                break;
            } else {
                s.push(tkn);
            }
        }

        tokens.push(Token::String(s.iter().collect()));

        return Ok(());
    }

    if ch.is_ascii_digit() {
        let mut buf = Vec::new();

        buf.push(ch);

        while let Some(tkn) = cursor.peek() {
            if tkn.is_ascii_digit() || (tkn == '.' && !buf.contains(&'.')) {
                buf.push(tkn);
                cursor.skip(1);
            } else {
                break;
            }
        }

        if buf.contains(&'.') {
            if let Ok(it) = buf.iter().collect::<String>().parse() {
                tokens.push(Token::Float(it));
            } else {
                return Err(TokenizerError {
                    src: cursor.source(),
                    at: cursor.span(buf.len()),
                    err: format!(
                        "Could not parse as a float: {}",
                        buf.iter().collect::<String>()
                    ),
                });
            }
        } else {
            if let Ok(it) = buf.iter().collect::<String>().parse() {
                tokens.push(Token::Int(it));
            } else {
                return Err(TokenizerError {
                    src: cursor.source(),
                    at: cursor.span(buf.len()),
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

        ident.push(ch);

        while let Some(tkn) = cursor.peek() {
            if tkn.is_ascii_alphanumeric() || tkn == '_' {
                ident.push(tkn);
                cursor.skip(1);
            } else {
                break;
            }
        }

        tokens.push(Token::Ident(ident.iter().collect()));

        return Ok(());
    }

    Err(TokenizerError {
        src: cursor.source(),
        at: cursor.span(1),
        err: format!("Unexpected token: {}", ch),
    })
}

pub fn tokenize(
    file: impl AsRef<str>,
    data: impl AsRef<str>,
) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    let mut cursor = Cursor::new(file, data);

    while let Some(ch) = cursor.next() {
        tokenize_inner(ch, &mut cursor, &mut tokens)?;
    }

    Ok(tokens)
}
