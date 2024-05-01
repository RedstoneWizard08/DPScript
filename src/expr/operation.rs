use serde::{Deserialize, Serialize};

use crate::{
    error::CompilationError, expr::Expr, lines::LineBuilder, source, state::State, Result,
    DPSCRIPT_TEMP_STORE,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    pub lhs: Box<Expr>,
    pub op: String,
    pub rhs: Box<Expr>,
}

impl Operation {
    pub fn get_type(&self, state: &State) -> Result<String> {
        let lhs_ty = self.lhs.get_type(state)?;
        let rhs_ty = self.rhs.get_type(state)?;

        if lhs_ty == rhs_ty {
            Ok(lhs_ty)
        } else if (lhs_ty == "Component" && rhs_ty == "Nbt")
            || (lhs_ty == "Nbt" && rhs_ty == "Component")
        {
            Ok("Component".into())
        } else if (lhs_ty == "Int" && rhs_ty == "Float") || (lhs_ty == "Float" && rhs_ty == "Int") {
            Ok("Float".into())
        } else {
            Err(CompilationError {
                src: source!(state),
                err: format!(
                    "Cannot perform operation {} on {:?} and {:?}!",
                    self.op, self.lhs, self.rhs
                ),
            }
            .into())
        }
    }

    pub fn compile(&self, state: &mut State, temp: impl AsRef<str>) -> Result<String> {
        let mut b = LineBuilder::new();
        let lhs_id = "__op_lhs";
        let rhs_id = "__op_rhs";
        let lhs_ty = self.lhs.get_type(state)?;
        let rhs_ty = self.rhs.get_type(state)?;

        if (lhs_ty == "Component" && rhs_ty == "Nbt") || (lhs_ty == "Nbt" && rhs_ty == "Component")
        {
            b.push(format!(
                "data modify storage {} {} set value {}",
                DPSCRIPT_TEMP_STORE,
                temp.as_ref(),
                self.lhs.compile(state, "")?
            ));

            b.push(format!(
                "data merge storage {} {{\"{}\": {}}}",
                DPSCRIPT_TEMP_STORE,
                temp.as_ref(),
                self.rhs.compile(state, "")?
            ));

            return Ok(b.build());
        }

        if self.lhs.is_value() {
            b.push(format!(
                "data modify storage {} {} set value {}",
                DPSCRIPT_TEMP_STORE,
                lhs_id,
                self.lhs.compile(state, "")?
            ));
        } else {
            b.push(self.lhs.compile(state, lhs_id)?);
        }

        if self.rhs.is_value() {
            b.push(format!(
                "data modify storage {} {} set value {}",
                DPSCRIPT_TEMP_STORE,
                rhs_id,
                self.rhs.compile(state, "")?
            ));
        } else {
            b.push(self.rhs.compile(state, rhs_id)?);
        }

        const MATH_OPS: [&str; 8] = ["+", "-", "*", "/", "**", "&", "|", "^"];

        if MATH_OPS.contains(&self.op.as_str()) {
            let op = match self.op.as_str() {
                "+" => "add",
                "-" => "sub",
                "*" => "mul",
                "/" => "div",
                "**" => "pow",
                "&" => "and",
                "|" => "or",
                "^" => "xor",

                _ => unreachable!(),
            };

            b.push(format!(
                "data operation storage {} {} {} {} {}",
                DPSCRIPT_TEMP_STORE, lhs_id, op, DPSCRIPT_TEMP_STORE, rhs_id
            ));

            b.push(format!(
                "data modify storage {} {} set from storage {} {}",
                DPSCRIPT_TEMP_STORE,
                temp.as_ref(),
                DPSCRIPT_TEMP_STORE,
                lhs_id
            ));

            Ok(b.build())
        } else {
            todo!("Conditional Operations")
        }
    }
}
