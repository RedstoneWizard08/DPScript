use std::{fs, path::PathBuf};

use clap::Parser;
use dpscript::tokenizer::tokenize;
use miette::IntoDiagnostic;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub file: PathBuf,
}

pub fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let data = fs::read_to_string(&cli.file).into_diagnostic()?;
    let tokens = tokenize(&cli.file.to_str().unwrap(), data)?;

    println!(
        "{}",
        tokens
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(" ")
    );

    Ok(())
}
