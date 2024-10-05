use std::{fs, path::PathBuf};

use clap::Parser;
use dpscript::{tokenizer::tokenize, Lexer};
use miette::IntoDiagnostic;
use ron::ser::PrettyConfig;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub file: PathBuf,
}

pub fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let file = cli.file.to_str().unwrap();
    let data = fs::read_to_string(&cli.file).into_diagnostic()?;
    let tokens = tokenize(&file, data.clone())?;

    let new_file = cli.file.with_extension("dps.tokens.ron");

    fs::write(
        new_file,
        ron::ser::to_string_pretty(&tokens, PrettyConfig::new()).into_diagnostic()?,
    )
    .into_diagnostic()?;

    let new_file = cli.file.with_extension("dps.ast.ron");
    let ast = Lexer::new(&file, data, tokens).run()?.get_ast();

    fs::write(
        new_file,
        ron::ser::to_string_pretty(&ast, PrettyConfig::new()).into_diagnostic()?,
    )
    .into_diagnostic()?;

    Ok(())
}
