use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

use crate::{compiler::compile, config::PackToml, parser::parse, state::State, Result};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Build {
        #[arg(short, long = "config", default_value_os_t = PathBuf::from("./pack.toml"))]
        config_path: PathBuf,
    },
}

impl Cli {
    pub fn start() -> Result<()> {
        Self::parse().run()
    }

    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Build { config_path } => {
                let config = fs::read_to_string(config_path)?;
                let config = toml::from_str::<PackToml>(&config)?;
                let root = config_path.parent().unwrap();
                let mut state = State::new(config, root.into());

                let walk = WalkDir::new(root)
                    .into_iter()
                    .filter_map(|v| v.ok())
                    .filter(|v| v.file_name().to_str().unwrap().ends_with(".dps"))
                    .collect::<Vec<_>>();

                let bar = ProgressBar::new(walk.len() as u64).with_style(
                    ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos:.blue} of {len:.blue}")
                        .unwrap()
                        .progress_chars("## "),
                );

                for entry in walk {
                    let path = entry.path();
                    let path_str = path.strip_prefix(root).unwrap().to_str().unwrap();

                    bar.println(format!(
                        "{} {}",
                        style("[compile]").blue(),
                        style(path_str).magenta()
                    ));

                    self.compile(&mut state, path_str, path.into())?;

                    bar.inc(1);
                }

                bar.finish();
            }
        }

        Ok(())
    }

    pub fn compile(
        &self,
        state: &mut State,
        file_name: impl AsRef<str>,
        file: PathBuf,
    ) -> Result<()> {
        let source = fs::read_to_string(file)?;
        let data = parse(&file_name, &source)?;

        compile(state, file_name, source, data)?;

        Ok(())
    }
}
