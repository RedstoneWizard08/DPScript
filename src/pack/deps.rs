use std::{fs, path::PathBuf};

use miette::{NamedSource, SourceOffset, SourceSpan};
use walkdir::WalkDir;

use crate::{DependencyError, Result};

use super::PackToml;

pub fn get_source_files(dir: &PathBuf, pack: PackToml) -> Result<Vec<(String, String)>> {
    let mut files = Vec::new();

    files.extend(get_pack_source_files(dir));

    files.extend(
        resolve_deps(pack)?
            .iter()
            .map(get_pack_source_files)
            .flatten()
            .collect::<Vec<_>>(),
    );

    Ok(files)
}

fn get_pack_source_files(dir: &PathBuf) -> Vec<(String, String)> {
    let root = dir.join("src");
    let mut files = Vec::new();

    let walk = WalkDir::new(&root)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|v| v.file_name().to_str().unwrap().ends_with(".dps"))
        .collect::<Vec<_>>();

    for entry in walk {
        let path = entry.path();
        let path_str = path.strip_prefix(&root).unwrap().to_str().unwrap();

        files.push((path.to_str().unwrap().into(), path_str.into()));
    }

    files
}

fn resolve_deps(proj: PackToml) -> Result<Vec<PathBuf>> {
    let mut dep_dirs = Vec::new();

    for (item, path) in proj.dependencies {
        let path = PathBuf::from(path);
        let file = path.join("pack.toml");
        let data = fs::read_to_string(&file)?;
        let toml = toml::from_str::<PackToml>(&data)?;

        if toml.pack.name != item {
            let src = NamedSource::new(file.to_str().unwrap(), data.clone());

            return Err(DependencyError {
                src,
                at: SourceSpan::new(SourceOffset::from_location(data, 0, 0), 1),
                err: format!(
                    "Dependency named \"{}\" does not match required name \"{}\"!",
                    toml.pack.name, item
                ),
            }
            .into());
        }

        let path = path.canonicalize()?;

        if !dep_dirs.contains(&path) {
            dep_dirs.push(path);
        }

        for item in resolve_deps(toml)? {
            if !dep_dirs.contains(&item) {
                dep_dirs.push(item);
            }
        }
    }

    Ok(dep_dirs)
}