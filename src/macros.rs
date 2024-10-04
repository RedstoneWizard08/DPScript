#[macro_export]
macro_rules! check_token {
    ($cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::Token::$tkn {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                });
            }
        }
    }};

    (remove $cursor: ident => $var: ident[$n: expr] == $tkn: ident) => {{
        let it = $var.get($n);

        if let Some((token, span)) = it {
            if token.clone() != $crate::Token::$tkn {
                return Err($crate::ParserError {
                    src: $cursor.source(),
                    at: span.clone(),
                    err: format!("Unexpected token: {}", token),
                });
            }
        }

        $var.remove($n);
    }};
}
