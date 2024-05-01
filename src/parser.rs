use std::collections::HashMap;

use miette::{NamedSource, SourceOffset, SourceSpan};
use peg::parser;

use crate::{
    call::Call,
    comp::Component,
    error::ParseError,
    expr::Expr,
    func::Func,
    nbt::{Nbt, NbtItem},
    op::Operation,
    selector::Selector,
    var::Var,
};

// TODO: Booleans
parser! {
    pub grammar dpscript() for str {
        rule _() = quiet!{ [' ' | '\t' | '\n']* }

        rule int__() -> u32
            = n: $(['0'..='9']+)
            {? n.parse().or(Err("u32")) }

        rule int_() -> i32
            = m: (['-' | '+'])? _ v: int__()
            { if m == Some('-') { v as i32 * -1 } else { v as i32 } }

        pub rule int() -> Expr
            = v: int_()
            { Expr::Int(v) }

        pub rule int_item() -> NbtItem
            = v: int_()
            { NbtItem::Int(v) }

        rule float_() -> f32
            = s: int_() _ "." _ e: int_()
            {? format!("{}.{}", s, e).parse().or(Err("f32")) }

        pub rule float() -> Expr
            = v: float_()
            { Expr::Float(v) }

        pub rule float_item() -> NbtItem
            = v: float_()
            { NbtItem::Float(v) }

        pub rule num() -> Expr
            = float() / int()

        pub rule num_item() -> NbtItem
            = float_item() / int_item()

        rule str_() -> String
            = "\"" _ v: $([^'"']+) _ "\""
            { v.to_owned() }

        pub rule str() -> Expr
            = v: str_()
            { Expr::String(v) }

        pub rule str_item() -> NbtItem
            = v: str_()
            { NbtItem::String(v) }

        rule component_str() -> Component
            = "c#" t: str_()
            { Component::new(t) }

        rule component_var() -> Component
            = "component" _ "(" _ e: operation() _ ")"
            { Component::from_expr(e) }

        rule component_() -> Component
            = component_var() / component_str()

        pub rule component() -> Expr
            = v: component_()
            { Expr::Component(v) }

        rule ident_() -> String
            = quiet!{
                n: $(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
                { n.to_owned() }
            } / expected!("identifier")

        pub rule ident() -> Expr
            = v: ident_()
            { Expr::Ident(v) }

        pub rule ident_item() -> NbtItem
            = v: ident_()
            { NbtItem::Ident(v) }

        rule nbt_item() -> NbtItem
            = str_item() / num_item() / map_as_item() / array_item() / ident_item()

        rule nbt_array() -> Vec<NbtItem>
            = "[" _ v: (nbt_item() ** ("," _)) _ "]"
            { v }

        pub rule array_item() -> NbtItem
            = v: nbt_array()
            { NbtItem::Array(Box::new(v)) }

        rule array_() -> Vec<Expr>
            = "[" _ v: (operation() ** ("," _)) _ "]"
            { v }

        pub rule array() -> Expr
            = v: array_()
            { Expr::Array(v) }

        pub rule map() -> HashMap<String, NbtItem>
            = "{" _ v: ((k: ident_() _ ":" _ v: nbt_item() { (k, v) }) ** ("," _)) _ "}"
            { HashMap::from_iter(v) }

        pub rule map_as_item() -> NbtItem
            = v: map()
            { NbtItem::Map(Box::new(v)) }

        rule at_selectors() -> String
            = "@" _ s: $(['s' | 'e' | 'a' | 'p' | 'r'])
            { format!("@{}", s) }

        rule selector_str() -> String
            = at_selectors() / str_()

        rule selector_() -> Expr
            = "entity" _ "(" _ s: selector_str() _ ")" _ d: (map())?
            { Expr::Selector(Selector { entity: s, params: d.unwrap_or_default() }) }

        rule at_selector_() -> Expr
            = v: at_selectors()
            { Expr::Selector(Selector { entity: v, params: HashMap::new() }) }

        pub rule selector() -> Expr
            = at_selector_() / selector_()

        rule nbt_() -> Nbt
            = "nbt" _ t: ("(" _ t: ident_() _ ")" { t })? _ v: map()
            { Nbt { ty: t, data: v } }

        pub rule nbt() -> Expr
            = v: nbt_()
            { Expr::Nbt(v) }

        rule var_() -> Var
            = "let" _ n: ident_() _ ty: (":" _ v: ident_() { v })?
            _ "=" _ v: operation()
            { Var { name: n, ty, is_const: false, value: Box::new(v) } }

        pub rule var() -> Expr
            = v: var_()
            { Expr::Var(v) }

        rule const_() -> Var
            = "const" _ n: ident_() _ ty: (":" _ v: ident_() { v })?
            _ "=" _ v: operation() _ ";"
            { Var { name: n, ty, is_const: true, value: Box::new(v) } }

        pub rule const_var() -> Expr
            = v: const_()
            { Expr::Var(v) }

        rule call_(is_command: bool) -> Call
            = n: ident_() _ "(" _ a: (operation() ** ("," _)) _ ")"
            { Call { func: n, args: a, is_command } }

        pub rule call() -> Expr
            = v: call_(false)
            { Expr::Call(v) }

        rule command_() -> Call
            = "/" _ v: call_(true)
            { v }

        pub rule command() -> Expr
            = v: command_()
            { Expr::Command(v) }

        rule function_() -> Func
            = c: ("#" _ "[" _ "name" _ "=" _ c: str_() _ "]" { c })?
            _ "fn" _ n: ident_()
            _ "(" _ a: ((n: ident_() _ ":" _ t: ident_() { (n, t) }) ** ("," _)) _ ")"
            _ r: ("-" _ ">" _ r: ident_() { r })?
            _ "{" _ b: ((_ v: expr() _ ";" { v }) / comment())* _ "}"
            { Func { name: n, custom_name: c, ret: r, args: a, body: b } }

        pub rule function() -> Expr
            = v: function_()
            { Expr::Func(v) }

        rule import_() -> String
            = "import" _ p: ident_() _ ";"
            { p }

        pub rule import() -> Expr
            = v: import_()
            { Expr::Import(v) }

        pub rule comment() -> Expr
            = _ "//" _ (!"\n" [_])*
            { Expr::None }

        pub rule blank_line() -> Expr
            = "\n"
            { Expr::None }

        pub rule return_() -> Expr
            = "return" _ v: operation()
            { Expr::Return(Box::new(v)) }

        pub rule operation() -> Expr = v: precedence! {
            l: (@) _ "==" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "==".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "!=" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "!=".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "<=" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "<=".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ ">=" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: ">=".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "<" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "<".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ ">" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: ">".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "&&" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "&&".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "||" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "||".into(), rhs: Box::new(r)
                })
            }

            --

            l: (@) _ "&" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "&".into(), rhs: Box::new(r)
                })
            }

            --

            l: (@) _ "^" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "^".into(), rhs: Box::new(r)
                })
            }

            --

            l: (@) _ "|" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "|".into(), rhs: Box::new(r)
                })
            }

            --

            l: (@) _ "-" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "-".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "+" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "+".into(), rhs: Box::new(r)
                })
            }

            --

            l: (@) _ "/" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "/".into(), rhs: Box::new(r)
                })
            }

            l: (@) _ "*" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "*".into(), rhs: Box::new(r)
                })
            }

            --

            l: (@) _ "**" _ r: @ {
                Expr::Operation(Operation {
                    lhs: Box::new(l), op: "**".into(), rhs: Box::new(r)
                })
            }

            --

            v: value_expr() { v }
            "(" v: operation() ")" { v }
        }

        rule value_expr() -> Expr
            = component() / str() / num() / array() / selector() / nbt() / call() / ident()

        pub rule expr() -> Expr
            = return_() / command() / var() / operation() / blank_line()

        pub rule top_level_expr() -> Expr
            = comment() / import() / const_var() / function() / blank_line()

        // Begin parsing here!!
        pub rule parser() -> Vec<Expr>
            = top_level_expr()*
    }
}

pub fn parse(file: impl AsRef<str>, input: impl AsRef<str>) -> Result<Vec<Expr>, ParseError> {
    let input = input.as_ref();
    let src = NamedSource::new(file, input.to_string());

    match dpscript::parser(input) {
        Ok(val) => Ok(val
            .iter()
            .map(|v| v.fix())
            .filter(|v| v != &Expr::None)
            .collect()),

        Err(err) => {
            let offset = SourceOffset::from_location(input, err.location.line, err.location.column);
            let span = SourceSpan::new(offset, err.location.column);

            Err(ParseError {
                src,
                at: span,
                err: format!("Expected: {}", err.expected),
            })
        }
    }
}
