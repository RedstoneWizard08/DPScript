use dpscript::{analysis::Analyzer, tokenize, Attribute, Cursor};

pub fn main() -> miette::Result<()> {
    let file = "test.dps";
    let code = "#[cmd(abc)]";
    let tokens = tokenize(file, code)?;

    println!("{:?}", tokens);

    let mut cursor = Cursor::new_from_src(file, code, tokens);
    let analyzed = Attribute::analyze(cursor.next().unwrap(), &mut cursor);

    println!("{:#?}", analyzed);

    Ok(())
}
