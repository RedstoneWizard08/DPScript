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

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parsing(#[from] ParseError),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Compilation(#[from] CompilationError),

    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::unknown), url(docsrs))]
    Unknown(#[from] anyhow::Error),

    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::io), url(docsrs))]
    Io(#[from] std::io::Error),

    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::ron), url(docsrs))]
    Ron(#[from] ron::Error),

    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::toml), url(docsrs))]
    Toml(#[from] toml::de::Error),

    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::json), url(docsrs))]
    Josn(#[from] serde_json::Error),

    #[error("An error occured!")]
    #[diagnostic(code(dpscript::error::semver), url(docsrs))]
    Semver(#[from] semver::Error),

    #[error("Unknown Minecraft version!")]
    #[diagnostic(code(dpscript::error::mc_ver), url(docsrs))]
    UnknownVer(#[help] String),
}
