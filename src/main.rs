use clap::Parser;
use dpscript::Cli;

pub fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();
    Cli::parse().run()?;
    Ok(())
}
