use crate::{
    check_token, AddSpan, Attribute, Cursor, Function, FunctionArg, Node, ParserError,
    ParserResult, Spanned, Token, TokenCursor, Type,
};

use super::Analyzer;

impl Analyzer<Function> for Function {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Function>> {
        match item.0 {
            Token::Fn | Token::Pub | Token::Facade | Token::Component => {}
            _ => return Ok(None),
        }

        let attrs = Attribute::analyze(item.clone(), cursor, nodes)?;

        let attrs = if let Some(attrs) = attrs {
            item = cursor.next_or_die(item.1)?;
            vec![attrs]
        } else {
            Vec::new()
        };

        let is_pub = match item.0 {
            Token::Pub => true,
            Token::Fn | Token::Facade | Token::Compiler => false,
            _ => return Ok(None),
        };

        debug!("is_pub: {}", is_pub);

        if is_pub {
            if !cursor
                .peek()
                .is_some_and(|(v, _)| v == Token::Fn || v == Token::Facade || v == Token::Compiler)
            {
                return Ok(None);
            }

            item = cursor.next().unwrap();
        }

        let is_facade = match item.0 {
            Token::Facade => true,
            Token::Fn | Token::Compiler => false,
            _ => return Ok(None),
        };

        debug!("is_facade: {}", is_facade);

        if is_facade {
            check_token!(cursor == Fn);
            item = cursor.next().unwrap();
        }

        let is_compiler = match item.0 {
            Token::Compiler => true,
            Token::Fn => false,
            _ => return Ok(None),
        };

        debug!("is_compiler: {}", is_compiler);

        if is_compiler {
            check_token!(cursor == Fn);
            item = cursor.next().unwrap();
        }

        check_token!(cursor => item == Fn);

        let (name, name_span) = cursor.next_or_die(item.1)?;

        let name = match name {
            Token::Ident(id) => (id, name_span),

            _ => {
                return Err(ParserError {
                    src: cursor.source(),
                    at: name_span,
                    err: format!("Unexpected token while parsing a function: {}", name),
                })
            }
        };

        debug!("Found function with name: {}", name.0);

        let it = check_token!(remove cursor == LeftParen).unwrap();

        let mut args = Vec::new();
        let mut span = item.1.add(it.1);

        if cursor.peek().is_some_and(|(v, _)| v != Token::RightParen) {
            // cursor.skip(1);

            let mut buf = Vec::new();
            let mut opens = 0;

            while let Some((token, span)) = cursor.next() {
                if token == Token::LeftParen {
                    opens += 1;
                }

                if token == Token::RightParen {
                    if opens == 0 {
                        break;
                    } else {
                        opens -= 1;
                    }
                }

                buf.push((token, span));
            }

            let mut arg_cursor = TokenCursor::new_from_src(
                cursor.source().name(),
                cursor.source().inner().clone(),
                buf,
            );

            while let Some(token) = arg_cursor.next() {
                while let Some(arg) = FunctionArg::analyze(token.clone(), &mut arg_cursor, nodes)? {
                    args.push(arg);
                }
            }

            if let Some(arg) = args.last() {
                span = span.add(arg.span);
            }
        } else {
            span = span.add(check_token!(remove cursor == RightParen).unwrap().1);
        }

        let mut ret = None;

        if cursor.peek().is_some_and(|(v, _)| v == Token::Minus)
            && cursor
                .peek_ahead(1)
                .is_some_and(|(v, _)| v == Token::RightAngle)
        {
            debug!("Parsing a return type...");

            let (_, span_) = cursor.next_or_die(item.1)?;
            let (_, span_) = cursor.next_or_die(span_)?;

            ret = Type::analyze(cursor.next_or_die(span_)?, cursor, nodes)?;

            if let Some(ref ret) = ret {
                span = span.add(ret.span);
            };
        }

        let mut body = Vec::new();

        if !is_facade && !is_compiler {
            check_token!(remove cursor == LeftBrace);

            let mut buf = Vec::new();
            let mut opens = 0;

            while let Some((token, span)) = cursor.next() {
                if token == Token::LeftBrace {
                    opens += 1;
                }

                if token == Token::RightBrace {
                    if opens == 0 {
                        break;
                    } else {
                        opens -= 1;
                    }
                }

                buf.push((token, span));
            }

            if let Some(tkn) = buf.last() {
                span = span.add(tkn.1);
            }

            let mut buf_cursor =
                Cursor::new_from_src(cursor.source().name(), cursor.source().inner().clone(), buf);

            while let Some(item) = buf_cursor.next() {
                Node::analyze(item, &mut buf_cursor, &mut body)?;
            }
        }

        Ok(Some(Self {
            args,
            attrs,
            body,
            is_compiler,
            is_facade,
            is_pub,
            name,
            ret,
            span,
        }))
    }
}

impl Analyzer<FunctionArg> for FunctionArg {
    fn analyze(
        mut item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<FunctionArg>> {
        let attr = match item.0 {
            Token::Hash => {
                let it = Some(Attribute::analyze(item, cursor, nodes)?);
                item = cursor.next().unwrap();
                it.flatten()
            }

            _ => None,
        };

        let name = match item.0 {
            Token::Ident(id) => (id, item.1),

            _ => {
                return Err(ParserError {
                    src: cursor.source(),
                    at: item.1,
                    err: format!("Unexpected token while parsing a function: {}", item.0),
                })
            }
        };

        let colon = check_token!(remove cursor == Colon).unwrap();

        let ty = match cursor.peek() {
            Some(item) => match Type::analyze(item.clone(), cursor, nodes)? {
                Some(ty) => ty,
                None => {
                    return Err(ParserError {
                        src: cursor.source(),
                        at: item.1,
                        err: "Function arguments require a type!".into(),
                    })
                }
            },

            _ => {
                return Err(ParserError {
                    src: cursor.source(),
                    at: colon.1,
                    err: "Unexpected end of file!".into(),
                })
            }
        };

        Ok(Some(Self {
            attrs: attr.map(|v| vec![v]).unwrap_or_default(),
            name,
            span: item.1.add(ty.span),
            ty,
        }))
    }
}