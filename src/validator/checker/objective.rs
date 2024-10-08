use super::{Checker, CheckerContext};
use crate::{Objective, Result, TSValidatorError};

impl Checker<Objective> for Objective {
    fn check(item: &mut Objective, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.current_module();
        let objectives = module.objectives();
        let imports = module.imported_objects()?;
        let mut occurances = 0;

        for it in objectives {
            if it.name.0 == item.name.0 {
                occurances += 1;

                if occurances > 1 {
                    return Err(TSValidatorError {
                        src: module.source.clone(),
                        at: item.name.1,
                        other: it.name.1,
                        err: format!("Duplicate identifier: {}", item.name.0),
                    }
                    .into());
                }
            }
        }

        for it in imports {
            if it.export.name() == item.name.0 {
                return Err(TSValidatorError {
                    src: module.source.clone(),
                    at: item.name.1,
                    other: it.export.span(),
                    err: format!("Duplicate identifier: {}", item.name.0),
                }
                .into());
            }
        }

        Ok(())
    }
}
