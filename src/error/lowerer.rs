use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during lowering!")]
#[diagnostic(code(dpscript::error::lowerer), url(docsrs))]
pub struct LowererError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during lowering!")]
#[diagnostic(code(dpscript::error::lowerer), url(docsrs))]
pub struct UnnamedLowererError {
    #[source_code]
    pub src: String,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}
