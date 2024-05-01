use serde::{Deserialize, Serialize};

use crate::{compiler::Compilable, expr::Expr, state::State, Result, DPSCRIPT_VAR_STORE};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Var {
    pub name: String,
    pub ty: Option<String>,
    pub value: Box<Expr>,
    pub is_const: bool,
}

impl Compilable for Var {
    fn compile(&self, state: &mut State) -> Result<String> {
        if self.is_const {
            return Ok(String::new());
        }

        let data = if self.value.is_value() {
            Ok(format!(
                "data modify storage {} {} set value {}",
                DPSCRIPT_VAR_STORE,
                self.name,
                self.value.compile(state, "")?
            ))
        } else {
            Ok(self.value.compile(state, &self.name)?)
        };

        state
            .locals
            .insert(self.name.clone(), (self.name.clone(), self.clone()));

        data
    }
}
