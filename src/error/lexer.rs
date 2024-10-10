use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while lexing!")]
#[diagnostic(code(dpscript::error::lexer), url(docsrs))]
pub struct LexerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while lexing!")]
#[diagnostic(code(dpscript::error::lexer), url(docsrs))]
pub struct UnnamedLexerError {
    #[source_code]
    pub src: String,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}
