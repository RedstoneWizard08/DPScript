use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Validator(#[from] ValidatorError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Ron(#[from] ron::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while parsing!")]
#[diagnostic(code(dpscript::error::parser), url(docsrs))]
pub struct ParserError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validation), url(docsrs))]
pub struct ValidatorError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}
