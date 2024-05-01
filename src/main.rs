use dpscript::cli::Cli;

pub fn main() -> miette::Result<()> {
    Cli::start()?;

    Ok(())
}
