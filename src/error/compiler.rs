use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during compilation!")]
#[diagnostic(code(dpscript::error::compiler), url(docsrs))]
pub struct CompilerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during compilation!")]
#[diagnostic(code(dpscript::error::compiler), url(docsrs))]
pub struct UnnamedCompilerError {
    #[source_code]
    pub src: String,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during compilation!")]
#[diagnostic(code(dpscript::error::compiler), url(docsrs))]
pub struct UnsourcedCompilerError {
    #[help]
    pub err: String,
}
