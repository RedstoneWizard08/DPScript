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
                }
                .into());
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
                }
                .into());
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
                }
                .into());
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
            }
            .into());
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
                }
                .into());
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
                }
                .into());
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
                }
                .into());
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
            }
            .into());
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

#[macro_export]
macro_rules! module_top_level_getter {
    ($fn: ident -> $ty: ident) => {
        impl $crate::Module {
            pub fn $fn(&self) -> Vec<$crate::$ty> {
                let mut items = Vec::new();

                if let Some(nodes) = &self.top_level {
                    for node in nodes {
                        if let $crate::TopLevelNode::$ty(item) = node {
                            items.push(item.clone());
                        }
                    }
                }

                items
            }
        }
    };
}

#[macro_export]
macro_rules! module_indexer_add {
    ($id: ident += ($name: ident, $module: ident)) => {
        if let Some(it) = $id.get_mut(&$name) {
            it.extend($module.$id());
        } else {
            $id.insert($name.clone(), $module.$id());
        }
    };
}

#[macro_export]
macro_rules! dump_ast_part {
    ($ast: ident.$id: ident => $dir: ident) => {
        if let Some(it) = $ast.$id {
            let path = $dir.join(format!("{}.ron", stringify!($id)));

            fs::write(path, ron::ser::to_string_pretty(&it, PrettyConfig::new())?)?;
        }
    };
}
