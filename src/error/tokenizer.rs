use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while tokenizing!")]
#[diagnostic(code(dpscript::error::tokenizer), url(docsrs))]
pub struct TokenizerError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while tokenizing!")]
#[diagnostic(code(dpscript::error::tokenizer), url(docsrs))]
pub struct UnnamedTokenizerError {
    #[source_code]
    pub src: String,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}
