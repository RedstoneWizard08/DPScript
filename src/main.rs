use std::fs;

use miette::{IntoDiagnostic, Result};
use dpscript::parser::parse;

pub fn main() -> Result<()> {
    let file = "examples/demo.dps";
    let data = fs::read_to_string(file).into_diagnostic()?;
    let data = parse(file, &data)?;

    println!("{:#?}", data);

    Ok(())
}
