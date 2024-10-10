use super::Checker;
use crate::{CheckerContext, DuoValidatorError, Loop, Node, Result, TypeKind, ValidatorError};

impl Checker<Loop> for Loop {
    fn check(item: &mut Loop, cx: &mut CheckerContext) -> Result<()> {
        let refs = cx.get_refs()?;
        let module = cx.current_module();

        let Some(found) = refs.get(&item.array.0) else {
            return Err(ValidatorError {
                src: module.source(),
                at: item.array.1,
                err: format!("Cannot find value: {}", item.array.0),
            }
            .into());
        };

        if !found.is_array(&module, cx)? {
            if found.get_type(&module, cx)?.kind != TypeKind::Selector {
                return Err(ValidatorError {
                    src: module.source(),
                    at: item.array.1,
                    err: format!("{} is not an array!", item.array.0),
                }
                .into());
            }
        }

        if let Some(orig) = refs.get(&item.var_name.0) {
            return Err(DuoValidatorError {
                src: module.source(),
                at: item.var_name.1,
                other: orig.span(),
                err: format!("Duplicate name {} already exists!", item.var_name.0),
            }
            .into());
        }

        cx.cur_loops.push(item.clone());

        for item in &mut item.body {
            Node::check(item, cx)?;
        }

        cx.cur_loops.pop();

        Ok(())
    }
}
