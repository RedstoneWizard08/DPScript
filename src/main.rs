use std::{fs, path::PathBuf};

use clap::Parser;
use dpscript::tokenizer::tokenize;
use miette::IntoDiagnostic;
use ron::ser::PrettyConfig;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub file: PathBuf,
}

pub fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let data = fs::read_to_string(&cli.file).into_diagnostic()?;
    let tokens = tokenize(&cli.file.to_str().unwrap(), data)?;

    // println!(
    //     "{}",
    //     tokens
    //         .iter()
    //         .map(|v| format!("{}", v.0))
    //         .collect::<Vec<_>>()
    //         .join(" ")
    // );

    // println!("{:#?}", tokens);

    let new_file = cli.file.with_extension("dps.tokens.ron");

    fs::write(
        new_file,
        ron::ser::to_string_pretty(&tokens, PrettyConfig::new()).into_diagnostic()?,
    )
    .into_diagnostic()?;

    Ok(())
}
