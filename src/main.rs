use std::fs;

use dpscript::{compiler::compile, config::PackToml, parser::parse, Result};
// use ron::ser::PrettyConfig;

pub fn run() -> Result<()> {
    let file = "examples/demo.dps";
    let config_file = "examples/pack.toml";

    let source = fs::read_to_string(file)?;
    let config_data = fs::read_to_string(config_file)?;

    let config = toml::from_str::<PackToml>(&config_data)?;
    let data = parse(file, &source)?;

    // fs::write(
    //     "examples/demo.ast.ron",
    //     ron::ser::to_string_pretty(&data, PrettyConfig::default())?,
    // )?;

    println!("Compiling...");

    let _state = compile(config, file, source, data)?;

    // fs::write(
    //     "examples/state.ron",
    //     ron::ser::to_string_pretty(&state, PrettyConfig::default())?,
    // )?;

    println!("Done!");

    Ok(())
}

pub fn main() -> miette::Result<()> {
    run()?;

    Ok(())
}
