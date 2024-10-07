use super::{ctx::CheckerContext, Checker};
use crate::{Import, ImportNode, Result, ValidatorError};

impl Checker<Import> for Import {
    fn check(item: &mut Import, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.current_module();

        let Some(target) = cx.modules.get(&item.module()) else {
            return Err(ValidatorError {
                src: module.source(),
                at: item.module_span(),
                err: format!("Cannot find module {}", item.module()),
            }
            .into());
        };

        let exports = target.get_exports(cx)?;

        for item in item.imports.clone() {
            match item {
                ImportNode::Object((name, span)) => {
                    let found = exports.iter().any(|v| v.name() == name);

                    if !found {
                        return Err(ValidatorError {
                            src: module.source(),
                            at: span,
                            err: format!("Object {} is either undefined for not public!", name),
                        }
                        .into());
                    }
                }

                _ => todo!("Nested imports"),
            }
        }

        Ok(())
    }
}
