use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Token {
    // =============== LITERALS ===============
    /// n
    Int(i64),

    /// n.n
    Float(f64),

    /// "\\"...\\""
    String(String),

    /// "true" or "false"
    Bool(bool),

    /// "[ident]"
    Ident(String),

    // =============== SYMBOLS ===============
    /// "&"
    And,

    /// ":"
    Colon,

    /// ","
    Comma,

    /// "["
    LeftBracket,

    /// "{"
    LeftBrace,

    /// "("
    LeftParen,

    /// "<"
    LeftAngle,

    /// "]"
    RightBracket,

    /// "}"
    RightBrace,

    /// ")"
    RightParen,

    /// ">"
    RightAngle,

    /// ";"
    Semi,

    /// "="
    Equal,

    /// "."
    Dot,

    /// "-"
    Minus,

    /// "+"
    Plus,

    /// "*"
    Star,

    /// "/"
    Slash,

    /// "#"
    Hash,

    // =============== GROUPS ===============
    /// "..."
    Ellipsis,

    /// ".."
    Range,

    // =============== KEYWORDS ===============
    /// "if"
    If,

    /// "in"
    In,

    /// "id"
    Id,

    /// "import"
    Import,

    // ========================================
    /// "store"
    Store,

    /// "selector"
    Selector,

    // ========================================
    /// "export"
    Export,

    /// "enum"
    Enum,

    // ========================================
    /// "fn"
    Fn,

    /// "for"
    For,

    /// "facade"
    Facade,

    // ========================================
    /// "pub"
    Pub,

    /// "path"
    Path,

    /// "player"
    Player,

    // ========================================
    /// "const"
    Const,

    /// "compiler"
    Compiler,

    /// "component"
    Component,

    // ========================================
    /// "let"
    Let,

    /// "return"
    Return,

    /// "objective"
    Objective,

    /// "module"
    Module,

    /// "tick"
    Tick,

    /// "init"
    Init,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Int(i) => write!(f, "{}", i),
            Self::Float(v) => write!(f, "{}", v),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Ident(i) => write!(f, "{}", i),
            Self::And => write!(f, "&"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::LeftBracket => write!(f, "["),
            Self::LeftBrace => write!(f, "{{"),
            Self::LeftParen => write!(f, "("),
            Self::LeftAngle => write!(f, "<"),
            Self::RightBracket => write!(f, "]"),
            Self::RightBrace => write!(f, "}}"),
            Self::RightParen => write!(f, ")"),
            Self::RightAngle => write!(f, ">"),
            Self::Semi => write!(f, ";"),
            Self::Equal => write!(f, "="),
            Self::Dot => write!(f, "."),
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Hash => write!(f, "#"),
            Self::Ellipsis => write!(f, "..."),
            Self::Range => write!(f, ".."),
            Self::If => write!(f, "if"),
            Self::In => write!(f, "in"),
            Self::Id => write!(f, "id"),
            Self::Import => write!(f, "import"),
            Self::Store => write!(f, "store"),
            Self::Selector => write!(f, "selector"),
            Self::Export => write!(f, "export"),
            Self::Enum => write!(f, "enum"),
            Self::Fn => write!(f, "fn"),
            Self::For => write!(f, "for"),
            Self::Facade => write!(f, "facade"),
            Self::Pub => write!(f, "pub"),
            Self::Path => write!(f, "path"),
            Self::Player => write!(f, "player"),
            Self::Const => write!(f, "const"),
            Self::Compiler => write!(f, "compiler"),
            Self::Component => write!(f, "component"),
            Self::Let => write!(f, "let"),
            Self::Return => write!(f, "return"),
            Self::Objective => write!(f, "objective"),
            Self::Module => write!(f, "module"),
            Self::Tick => write!(f, "tick"),
            Self::Init => write!(f, "init"),
        }
    }
}
