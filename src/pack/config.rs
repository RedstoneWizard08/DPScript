use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PackToml {
    pub pack: PackConfig,
    pub version: VersionConfig,
    pub build: BuildConfig,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PackConfig {
    /// The name also functions as the namespace.
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub authors: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct VersionConfig {
    pub minecraft: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BuildConfig {
    pub output: String,
}
