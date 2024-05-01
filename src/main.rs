use std::fs;

use dpscript::{compiler::compile, config::PackToml, parser::parse};
use miette::{IntoDiagnostic, Result};
use ron::ser::PrettyConfig;

pub fn main() -> Result<()> {
    let file = "examples/demo.dps";
    let config_file = "examples/pack.toml";

    let source = fs::read_to_string(file).into_diagnostic()?;
    let config_data = fs::read_to_string(config_file).into_diagnostic()?;

    let config = toml::from_str::<PackToml>(&config_data).into_diagnostic()?;
    let data = parse(file, &source)?;

    fs::write(
        "examples/demo.ast.ron",
        ron::ser::to_string_pretty(&data, PrettyConfig::default()).into_diagnostic()?,
    )
    .into_diagnostic()?;

    println!("Compiling...");

    let (code, state) = compile(config, file, source, data)?;

    fs::write("examples/demo.mcfunction", code).into_diagnostic()?;

    fs::write(
        "examples/state.ron",
        ron::ser::to_string_pretty(&state, PrettyConfig::default()).into_diagnostic()?,
    )
    .into_diagnostic()?;

    println!("Done!");

    Ok(())
}
