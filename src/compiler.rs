use std::fs;

use crate::{expr::Expr, format::generate_manifest, state::State, Result};

pub trait Compilable {
    fn compile(&self, state: &mut State) -> Result<String>;
}

pub fn compile(
    state: &mut State,
    file: impl AsRef<str>,
    source: impl AsRef<str>,
    exprs: Vec<Expr>,
) -> Result<()> {
    state.update(file, source, exprs.clone())?;

    for item in &exprs {
        if let Expr::Var(v) = item {
            if v.is_const {
                continue;
            }
        }

        item.compile(state, "")?;
    }

    if !exprs.is_empty() {
        let manifest = generate_manifest(&state.config)?;
        let file = state.out_dir.join("pack.mcmeta");

        fs::write(file, manifest)?;
    }

    Ok(())
}
