use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::basic), url(docsrs))]
    Basic(#[help] String),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] ParserError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    IRParser(#[from] IRParserError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Validator(#[from] ValidatorError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    TSValidator(#[from] TSValidatorError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Dependency(#[from] DependencyError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Lowering(#[from] LoweringError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Ron(#[from] ron::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[error(transparent)]
    Json5(#[from] json5::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
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
#[error("An error occured while parsing IR!")]
#[diagnostic(code(dpscript::error::parser::ir), url(docsrs))]
pub struct IRParserError {
    #[source_code]
    pub src: String,

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

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured while validating!")]
#[diagnostic(code(dpscript::error::validation), url(docsrs))]
pub struct TSValidatorError {
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
#[error("An error occured during dependency resolution!")]
#[diagnostic(code(dpscript::error::validation), url(docsrs))]
pub struct DependencyError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}

#[derive(Debug, Error, Diagnostic)]
#[error("An error occured during lowering!")]
#[diagnostic(code(dpscript::error::lowering), url(docsrs))]
pub struct LoweringError {
    #[source_code]
    pub src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,

    #[help]
    pub err: String,
}
