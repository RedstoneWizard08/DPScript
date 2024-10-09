use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum IRToken {
    // =============== LITERALS ===============
    /// "'...'"
    Literal(String),

    /// "[id]"
    Ident(String),

    /// n
    Int(i64),

    // =============== SYMBOLS ===============
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

    /// "]"
    RightBracket,

    /// "}"
    RightBrace,

    /// ")"
    RightParen,

    /// ";"
    Semi,

    /// "."
    Dot,

    /// "+"
    Plus,

    /// "$"
    Dollar,

    /// "!"
    Exclamation,

    /// "@"
    At,

    // =============== KEYWORDS ===============
    /// "append"
    Append,

    /// "argument"
    Argument,

    // ========================================
    /// "call"
    Call,

    /// "copy"
    Copy,

    /// "clear"
    Clear,

    /// "command"
    Command,

    /// "condition"
    Condition,

    // ========================================
    /// "data"
    Data,

    /// "define"
    Define,

    // ========================================
    /// "set"
    Set,

    /// "store"
    Store,

    // ========================================
    /// "else"
    Else,

    /// "execute"
    Execute,

    // ========================================
    /// "get"
    Get,

    /// "goto"
    Goto,

    // ========================================
    /// "func"
    Func,

    /// "path"
    Path,

    /// "tag"
    Tag,

    /// "variable_alias"
    VariableAlias,

    /// "if"
    If,

    /// "join"
    Join,
}

impl fmt::Display for IRToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Int(it) => write!(f, "{}", it),
            Self::Literal(it) => write!(f, "'{}'", it),
            Self::Ident(it) => write!(f, "{}", it),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::LeftBracket => write!(f, "["),
            Self::LeftBrace => write!(f, "{{"),
            Self::LeftParen => write!(f, "("),
            Self::RightBracket => write!(f, "]"),
            Self::RightBrace => write!(f, "}}"),
            Self::RightParen => write!(f, ")"),
            Self::Semi => write!(f, ";"),
            Self::Dot => write!(f, "."),
            Self::Plus => write!(f, "+"),
            Self::Dollar => write!(f, "$"),
            Self::Exclamation => write!(f, "!"),
            Self::At => write!(f, "@"),
            Self::Append => write!(f, "append"),
            Self::Argument => write!(f, "argument"),
            Self::Call => write!(f, "call"),
            Self::Copy => write!(f, "copy"),
            Self::Clear => write!(f, "clear"),
            Self::Command => write!(f, "command"),
            Self::Condition => write!(f, "condition"),
            Self::Data => write!(f, "data"),
            Self::Define => write!(f, "define"),
            Self::Set => write!(f, "set"),
            Self::Store => write!(f, "store"),
            Self::Else => write!(f, "else"),
            Self::Execute => write!(f, "execute"),
            Self::Func => write!(f, "func"),
            Self::Get => write!(f, "get"),
            Self::Goto => write!(f, "goto"),
            Self::Path => write!(f, "path"),
            Self::Tag => write!(f, "tag"),
            Self::VariableAlias => write!(f, "variable_alias"),
            Self::If => write!(f, "if"),
            Self::Join => write!(f, "join"),
        }
    }
}
