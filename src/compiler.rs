use crate::{config::PackToml, expr::Expr, lines::LineBuilder, state::State, Result};

pub trait Compilable {
    fn compile(&self, state: &mut State) -> Result<String>;
}

pub fn compile(
    config: PackToml,
    file: impl AsRef<str>,
    source: impl AsRef<str>,
    exprs: Vec<Expr>,
) -> Result<(String, State)> {
    let mut state = State::from_root(config, file, source, exprs.clone());
    let mut lines = LineBuilder::new();

    for item in exprs {
        lines.push(item.compile(&mut state, "")?);
    }

    // TODO: Not replace here
    Ok((lines.build().replace("  ", " "), state))
}
