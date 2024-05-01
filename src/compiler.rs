use std::fs;

use crate::{config::PackToml, expr::Expr, format::generate_manifest, state::State, Result};

pub trait Compilable {
    fn compile(&self, state: &mut State) -> Result<String>;
}

pub fn compile(
    config: PackToml,
    file: impl AsRef<str>,
    source: impl AsRef<str>,
    exprs: Vec<Expr>,
) -> Result<State> {
    let mut state = State::from_root(config.clone(), file, source, exprs.clone())?;

    for item in exprs {
        if let Expr::Var(v) = &item {
            if v.is_const {
                continue;
            }
        }

        item.compile(&mut state, "")?;
    }

    let manifest = generate_manifest(&config)?;
    let file = state.out_dir.join("pack.mcmeta");

    fs::write(file, manifest)?;

    Ok(state)
}
