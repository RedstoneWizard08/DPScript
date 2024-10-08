use super::{IRToken, IRTokenizer};
use crate::{IRParserError, IsNotIdent, Result};

impl IRTokenizer {
    pub(super) fn tokenize_inner(&mut self, ch: char) -> Result<()> {
        if ch.is_whitespace() {
            return Ok(());
        }

        if ch == '/' {
            if self.cursor.peek_ahead(0).is_some_and(|v| v == '/') {
                // println!("==>>> SKIPPING COMMENT");

                while let Some(ch) = self.cursor.next() {
                    if ch == '\n' {
                        break;
                    }
                }

                return Ok(());
            }
        }

        let sym = match ch {
            ':' => Some((IRToken::Colon, self.cursor.span(1))),
            ',' => Some((IRToken::Comma, self.cursor.span(1))),
            '[' => Some((IRToken::LeftBracket, self.cursor.span(1))),
            '{' => Some((IRToken::LeftBrace, self.cursor.span(1))),
            '(' => Some((IRToken::LeftParen, self.cursor.span(1))),
            ']' => Some((IRToken::RightBracket, self.cursor.span(1))),
            '}' => Some((IRToken::RightBrace, self.cursor.span(1))),
            ')' => Some((IRToken::RightParen, self.cursor.span(1))),
            ';' => Some((IRToken::Semi, self.cursor.span(1))),
            '.' => Some((IRToken::Dot, self.cursor.span(1))),
            '+' => Some((IRToken::Plus, self.cursor.span(1))),
            '$' => Some((IRToken::Dollar, self.cursor.span(1))),
            '!' => Some((IRToken::Exclamation, self.cursor.span(1))),
            '@' => Some((IRToken::At, self.cursor.span(1))),

            _ => None,
        };

        if let Some(tkn) = sym {
            self.tokens.push(tkn);
            return Ok(());
        }

        let kw = match ch {
            'a' => {
                if self.cursor.peek_many(0, 5).is_some_and(|v| v == "ppend")
                    && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(6);
                    self.cursor.skip(5);
                    Some((IRToken::Append, span))
                } else if self.cursor.peek_many(0, 7).is_some_and(|v| v == "rgument")
                    && self.cursor.peek_ahead(7).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(8);
                    self.cursor.skip(7);
                    Some((IRToken::Argument, span))
                } else {
                    None
                }
            }

            'c' => {
                if self.cursor.peek_many(0, 3).is_some_and(|v| v == "all")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((IRToken::Call, span))
                } else if self.cursor.peek_many(0, 3).is_some_and(|v| v == "opy")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((IRToken::Copy, span))
                } else if self.cursor.peek_many(0, 4).is_some_and(|v| v == "lear")
                    && self.cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(5);
                    self.cursor.skip(4);
                    Some((IRToken::Clear, span))
                } else if self.cursor.peek_many(0, 6).is_some_and(|v| v == "ommand")
                    && self.cursor.peek_ahead(6).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(7);
                    self.cursor.skip(6);
                    Some((IRToken::Command, span))
                } else {
                    None
                }
            }

            'd' => {
                if self.cursor.peek_many(0, 3).is_some_and(|v| v == "ata")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((IRToken::Data, span))
                } else if self.cursor.peek_many(0, 5).is_some_and(|v| v == "efine")
                    && self.cursor.peek_ahead(5).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(6);
                    self.cursor.skip(5);
                    Some((IRToken::Define, span))
                } else {
                    None
                }
            }

            's' => {
                if self.cursor.peek_many(0, 4).is_some_and(|v| v == "tore")
                    && self.cursor.peek_ahead(4).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(5);
                    self.cursor.skip(4);
                    Some((IRToken::Store, span))
                } else if self.cursor.peek_many(0, 2).is_some_and(|v| v == "et")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((IRToken::Set, span))
                } else {
                    None
                }
            }

            'e' => {
                if self.cursor.peek_many(0, 6).is_some_and(|v| v == "xecute")
                    && self.cursor.peek_ahead(6).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(7);
                    self.cursor.skip(6);
                    Some((IRToken::Execute, span))
                } else {
                    None
                }
            }

            'f' => {
                if self.cursor.peek_many(0, 3).is_some_and(|v| v == "unc")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((IRToken::Func, span))
                } else {
                    None
                }
            }

            'g' => {
                if self.cursor.peek_many(0, 2).is_some_and(|v| v == "et")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((IRToken::Get, span))
                } else {
                    None
                }
            }

            'p' => {
                if self.cursor.peek_many(0, 3).is_some_and(|v| v == "ath")
                    && self.cursor.peek_ahead(3).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(4);
                    self.cursor.skip(3);
                    Some((IRToken::Path, span))
                } else {
                    None
                }
            }

            't' => {
                if self.cursor.peek_many(0, 2).is_some_and(|v| v == "ag")
                    && self.cursor.peek_ahead(2).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(3);
                    self.cursor.skip(2);
                    Some((IRToken::Tag, span))
                } else {
                    None
                }
            }

            'v' => {
                if self
                    .cursor
                    .peek_many(0, 13)
                    .is_some_and(|v| v == "ariable_alias")
                    && self.cursor.peek_ahead(13).is_some_and(|v| v.is_not_ident())
                {
                    let span = self.cursor.span(14);
                    self.cursor.skip(13);
                    Some((IRToken::VariableAlias, span))
                } else {
                    None
                }
            }

            _ => None,
        };

        if let Some(tkn) = kw {
            self.tokens.push(tkn);

            return Ok(());
        }

        if ch == '\'' {
            let mut s = Vec::new();
            let c2 = self.cursor.clone();

            while let Some(tkn) = self.cursor.next() {
                if tkn == '\'' {
                    break;
                } else {
                    s.push(tkn);
                }
            }

            self.tokens
                .push((IRToken::Literal(s.iter().collect()), c2.span(s.len() + 2)));

            return Ok(());
        }

        if ch == '"' {
            let mut s = Vec::new();
            let c2 = self.cursor.clone();

            while let Some(tkn) = self.cursor.next() {
                if tkn == '"' {
                    break;
                } else {
                    s.push(tkn);
                }
            }

            self.tokens
                .push((IRToken::Literal(s.iter().collect()), c2.span(s.len() + 2)));

            return Ok(());
        }

        if ch.is_ascii_digit() {
            let mut buf = Vec::new();
            let c2 = self.cursor.clone();

            buf.push(ch);

            while let Some(tkn) = self.cursor.peek() {
                if tkn.is_ascii_digit() {
                    buf.push(tkn);
                    self.cursor.skip(1);
                } else {
                    break;
                }
            }

            let span = c2.span(buf.len());

            if let Ok(it) = buf.iter().collect::<String>().parse() {
                self.tokens.push((IRToken::Int(it), span));
            } else {
                return Err(IRParserError {
                    src: self.cursor.source(),
                    at: span,
                    err: format!(
                        "Could not parse as an int: {}",
                        buf.iter().collect::<String>()
                    ),
                }
                .into());
            }

            return Ok(());
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut ident = Vec::new();
            let c2 = self.cursor.clone();

            ident.push(ch);

            while let Some(tkn) = self.cursor.peek() {
                if tkn.is_ascii_alphanumeric() || tkn == '_' {
                    ident.push(tkn);
                    self.cursor.skip(1);
                } else {
                    break;
                }
            }

            self.tokens
                .push((IRToken::Ident(ident.iter().collect()), c2.span(ident.len())));

            return Ok(());
        }

        Err(IRParserError {
            src: self.cursor.source(),
            at: self.cursor.span(1),
            err: format!("Unexpected character during tokenization: {}", ch),
        }
        .into())
    }
}
