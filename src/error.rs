use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while parsing!")]
#[diagnostic(code(dpscript::error::parsing), url(docsrs))]
pub struct ParseError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during compilation!")]
#[diagnostic(code(dpscript::error::compilation), url(docsrs))]
pub struct CompilationError {
    #[source_code]
    pub src: NamedSource<String>,

    #[help]
    pub err: String,
}
