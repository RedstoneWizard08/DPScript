use super::{Checker, CheckerContext};
use crate::{Result, ValidatorError, Variable};

impl Checker<Variable> for Variable {
    fn check(item: &mut Variable, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.current_module();

        if let Some(ty) = &item.ty {
            let mut ty_name = ty.kind.name();
            let types = module.available_type_names()?;

            if ty_name.ends_with("[]") {
                ty_name = ty_name.trim_end_matches("[]").to_string();
            }

            if !types.contains(&ty_name) {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: ty.span,
                    err: format!("Could not resolve type: {}", ty_name),
                }
                .into());
            }

            if let Some(val) = &mut item.value {
                let val_ty = val.get_type(&module, cx)?;

                if !val_ty.is_compatible(ty) {
                    return Err(ValidatorError {
                        src: module.source.clone(),
                        at: ty.span,
                        err: "Value type is incompatible with the defined type!".into(),
                    }
                    .into());
                }
            }
        }

        if item.is_const && item.value.is_none() {
            return Err(ValidatorError {
                src: module.source.clone(),
                at: item.span,
                err: "Constant variables must be initialized!".into(),
            }
            .into());
        }

        // TODO: Duplicate name checking

        Ok(())
    }
}
