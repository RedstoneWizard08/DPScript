use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validator), url(docsrs))]
pub struct ValidatorError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validator::duo), url(docsrs))]
pub struct DuoValidatorError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[label("here")]
    pub other: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validator), url(docsrs))]
pub struct UnnamedValidatorError {
    #[source_code]
    pub src: String,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validator::duo), url(docsrs))]
pub struct UnnamedDuoValidatorError {
    #[source_code]
    pub src: String,

    #[label("here")]
    pub at: SourceSpan,

    #[label("here")]
    pub other: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validator), url(docsrs))]
pub struct UnsourcedValidatorError {
    #[help]
    pub err: String,
}
