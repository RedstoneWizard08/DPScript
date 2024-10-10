use super::Checker;
use crate::{BuiltInTypes, Call, CheckerContext, ExportNode, Node, Result, ValidatorError};

impl Checker<Call> for Call {
    fn check(item: &mut Call, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.cur_modules.clone();
        let module = module.last().unwrap();
        let funcs = module.funcs();
        let imports = module.imported_objects()?;
        let refs = cx.get_refs()?;

        if let Some((name, span)) = item.parent.clone() {
            let mut parent = None;

            for (item, it) in refs {
                if name == item {
                    parent = Some(it);
                    break;
                }
            }

            if parent.is_none() {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: span,
                    err: format!("Cannot resolve object: {}", name),
                }
                .into());
            }

            let parent = parent.unwrap();
            let base_ty = BuiltInTypes::from(parent.get_type(module, cx)?);
            let methods = base_ty.methods();
            let mut method = None;

            for (name, args, ret) in methods {
                if name == item.function.0 {
                    method = Some((name, args, ret));
                    break;
                }
            }

            if method.is_none() {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: item.function.1,
                    err: format!(
                        "Object {} does not define a method: {}",
                        name, item.function.0
                    ),
                }
                .into());
            }

            let method = method.unwrap();

            let mut arg_types = Vec::new();
            let mut real_types = Vec::new();

            for arg in &mut item.args {
                Node::check(arg, cx)?;

                let ty = arg.get_type(module, cx)?;

                arg_types.push((BuiltInTypes::from(ty.clone()), ty.span));
            }

            for real in method.1 {
                real_types.push(real);
            }

            if arg_types.len() != real_types.len() {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: item.span,
                    err: format!(
                        "Expected {} arguments, got {}",
                        real_types.len(),
                        arg_types.len()
                    ),
                }
                .into());
            }

            for (i, (arg, span)) in arg_types.iter().enumerate() {
                let real = real_types.get(i).unwrap();

                if !real.is_compatible(arg) {
                    return Err(ValidatorError {
                        src: module.source.clone(),
                        at: *span,
                        err: format!("Incompatible types! Expected {}, got {}", real, arg),
                    }
                    .into());
                }
            }

            return Ok(());
        }

        let mut found = None;

        for func in funcs {
            if func.name.0 == item.function.0 {
                found = Some(func);
                break;
            }
        }

        if found.is_none() {
            for import in imports {
                if let ExportNode::Function(func) = import.export {
                    if func.name.0 == item.function.0 {
                        found = Some(func);
                        break;
                    }
                }
            }
        }

        if found.is_none() {
            return Err(ValidatorError {
                src: module.source.clone(),
                at: item.function.1,
                err: format!("Cannot resolve function: {}", item.function.0),
            }
            .into());
        }

        let mut arg_types = Vec::new();
        let mut real_types = Vec::new();

        for arg in &mut item.args {
            Node::check(arg, cx)?;
            arg_types.push(arg.get_type(module, cx)?);
        }

        for real in found.unwrap().args {
            real_types.push(real.ty);
        }

        if arg_types.len() != real_types.len() {
            return Err(ValidatorError {
                src: module.source.clone(),
                at: item.span,
                err: format!(
                    "Expected {} arguments, got {}",
                    real_types.len(),
                    arg_types.len()
                ),
            }
            .into());
        }

        for (i, arg) in arg_types.iter().enumerate() {
            let real = real_types.get(i).unwrap();

            if !real.is_compatible(arg) {
                return Err(ValidatorError {
                    src: module.source.clone(),
                    at: arg.span,
                    err: format!(
                        "Incompatible types! Expected {}, got {}",
                        real.kind, arg.kind
                    ),
                }
                .into());
            }
        }

        Ok(())
    }
}
