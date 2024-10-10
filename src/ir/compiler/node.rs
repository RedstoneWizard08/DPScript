use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::{IRCheckerContext, IRLiteral, IRNode, Result, UnsourcedCompilerError};

use super::TagData;

pub const FIXER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)([^:\s]+:[^\s]+)\s([^\s]+)\s\[").unwrap());

impl IRNode {
    pub fn compile(&self, cx: &IRCheckerContext, dir: &PathBuf) -> Result<String> {
        match self {
            Self::Function(func) => {
                let id = func.id.to_lowercase();
                let mut split = id.split(":");
                let ns = split.next().unwrap();
                let name = split.next().unwrap();

                let path = dir
                    .join("data")
                    .join(ns)
                    .join("functions")
                    .join(format!("{}.mcfunction", name));

                let root = path.parent().unwrap();

                if !root.exists() {
                    fs::create_dir_all(root)?;
                }

                let mut file = File::create(path)?;

                for item in &func.body {
                    let data = item.compile(cx, dir)?;

                    if data.is_empty() {
                        continue;
                    }

                    writeln!(file, "{}", data)?;
                }

                Ok(String::new())
            }

            Self::Tag(tag) => {
                let mut split = tag.name.split(":");
                let ns = split.next().unwrap();
                let name = split.next().unwrap();
                let path = dir.join("data").join(ns).join(format!("{}.json", name));
                let root = path.parent().unwrap();

                if !root.exists() {
                    fs::create_dir_all(root)?;
                }

                let mut values = Vec::new();

                for item in &tag.entries {
                    if !values.contains(item) {
                        values.push(item.clone());
                    }
                }

                let data = TagData {
                    replace: false,
                    values,
                };

                fs::write(path, serde_json::to_string_pretty(&data)?)?;

                Ok(String::new())
            }

            Self::Literal(lit) => match lit {
                IRLiteral::String(s) => Ok(s.clone()),
                IRLiteral::PathOf(_) | IRLiteral::StoreOf(_) => Err(UnsourcedCompilerError {
                    err: "path!() and store!() expressions are not allowed during compilation!"
                        .into(),
                }
                .into()),
            },

            Self::Group(group) => {
                let mut data = Vec::new();

                for item in group {
                    let out = item.compile(cx, dir)?;

                    if out.is_empty() {
                        continue;
                    }

                    data.push(out);
                }

                Ok(data.join("\n"))
            }

            Self::Command(cmd) => {
                let mut buf = Vec::new();

                for node in &cmd.cmd {
                    buf.push(node.compile(cx, dir)?);
                }

                Ok(FIXER_REGEX.replace(&buf.join(" "), "$1 $2[").to_string())
            }

            Self::None => Ok(String::new()),
            // These get ignored, they were only for checking and analysis
            Self::Definition(_) => Ok(String::new()),

            _ => Err(UnsourcedCompilerError {
                err: format!("Unexpected node for compilation: {:?}", self),
            }
            .into()),
        }
    }
}
