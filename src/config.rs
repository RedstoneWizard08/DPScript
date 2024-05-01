use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PackToml {
    pub pack: PackConfig,
    pub version: VersionConfig,
    pub build: BuildConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PackConfig {
    pub name: String,
    pub namespace: String,
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
