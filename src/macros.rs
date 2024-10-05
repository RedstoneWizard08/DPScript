#[macro_export]
macro_rules! check_token {
    ($cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::Token::$tkn {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                });
            }
        }

        it
    }};

    (data $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if let $crate::Token::$tkn(_) = token {
            } else {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                });
            }
        }

        it
    }};

    (remove $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::Token::$tkn {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                });
            }
        }

        $var.remove($n)
    }};

    ($cursor: ident => $tkn: ident == $expected: ident) => {{
        if $tkn.0 != $crate::Token::$expected {
            return Err($crate::ParserError {
                src: $cursor.source(),
                at: $tkn.1.clone(),
                err: format!(
                    "Unexpected token: {} (expected: {})",
                    $tkn.0,
                    $crate::Token::$expected
                ),
            });
        }
    }};

    (data $cursor: ident == $tkn: ident) => {{
        let it = $cursor.peek();

        if let Some((token, span)) = it {
            if let $crate::Token::$tkn(_) = token {
            } else {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                });
            }
        }

        it
    }};

    ($cursor: ident == $tkn: ident) => {{
        let it = $cursor.peek();

        if let Some((token, span)) = it.clone() {
            if token != $crate::Token::$tkn {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                });
            }
        }

        it
    }};

    (remove $cursor: ident == $tkn: ident) => {{
        let it = $cursor.next();

        if let Some((token, span)) = it.clone() {
            if token != $crate::Token::$tkn {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!(
                        "Unexpected token: {} (expected: {})",
                        token,
                        $crate::Token::$tkn
                    ),
                });
            }
        }

        it
    }};

    (data $cursor: ident => $tkn: ident == $expected: ident) => {{
        if let $crate::Token::$expected(_) = $tkn.0 {
        } else {
            return Err($crate::ParserError {
                src: $cursor.source(),
                at: $tkn.1.clone(),
                err: format!("Unexpected token: {}", $tkn.0),
            });
        }
    }};
}

#[macro_export]
macro_rules! add_return {
    ($arr: ident += $variant: ident($val: ident)) => {{
        let node = $crate::Node::$variant($val);
        $arr.push(node.clone());
        return Ok(Some(node));
    }};
}
