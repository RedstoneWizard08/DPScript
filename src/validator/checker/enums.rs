use super::{Checker, CheckerContext};
use crate::{Enum, Result, TSValidatorError, ValidatorError};

impl Checker<Enum> for Enum {
    fn check(item: &mut Enum, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.current_module();
        let enums = module.enums();
        let imports = module.imported_objects()?;
        let mut occurences = 0;

        for it in enums {
            if it.name.0 == item.name.0 {
                occurences += 1;
                
                if occurences > 1 {
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
                occurences += 1;

                if occurences > 1 {
                    return Err(TSValidatorError {
                        src: module.source.clone(),
                        at: item.name.1,
                        other: it.export.span(),
                        err: format!("Duplicate identifier: {}", item.name.0),
                    }
                    .into());
                }
            }
        }

        let mut items = Vec::new();

        for (entry, span) in &item.entries {
            if items.contains(entry) {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: span.clone(),
                    err: format!("Enum entries must be unique!"),
                }
                .into());
            } else {
                items.push(entry.clone());
            }
        }

        Ok(())
    }
}
