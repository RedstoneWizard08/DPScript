use crate::Spanned;
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    /// Is this public?
    pub is_pub: bool,

    /// The variable name
    pub name: Spanned<String>,

    /// The criteria for the objective
    pub criteria: Spanned<String>,

    /// The ID of the objective (the part in quotes, after the equals sign)
    pub id: Spanned<String>,

    /// The span
    pub span: SourceSpan,
}
