use super::Checker;
use crate::{BuiltInTypes, CheckerContext, Node, Operation, Result, ValidatorError};

impl Checker<Operation> for Operation {
    fn check(item: &mut Operation, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.current_module();

        Node::check(&mut item.lhs, cx)?;
        Node::check(&mut item.rhs, cx)?;

        let kind = item.kind.name();
        let lhs_ty = item.lhs.get_type(&module, cx)?;
        let rhs_ty = item.rhs.get_type(&module, cx)?;
        let builtin_lhs = BuiltInTypes::from(lhs_ty);
        let builtin_rhs = BuiltInTypes::from(rhs_ty);
        let lhs_methods = builtin_lhs.methods();
        let rhs_methods = builtin_rhs.methods();

        let lhs_method = lhs_methods
            .iter()
            .cloned()
            .find(|(name, _, _)| name == &kind);

        let rhs_method = rhs_methods
            .iter()
            .cloned()
            .find(|(name, _, _)| name == &kind);

        if lhs_method.is_none() && rhs_method.is_none() {
            return Err(ValidatorError {
                src: module.source(),
                at: item.lhs.span(),
                err: format!("Operands do not support the method: {}", kind),
            }
            .into());
        }

        let is_lhs = lhs_method.is_some();
        let operand = if is_lhs { builtin_lhs } else { builtin_rhs };
        let (_, args, _) = lhs_method.unwrap_or_else(|| rhs_method.unwrap());

        if args.len() > 1 {
            return Err(ValidatorError {
                src: module.source(),
                at: item.span,
                err: "Resolved method required more than one argument. If you're seeing this error, please create an issue on our GitHub!".into(),
            }
            .into());
        }

        if !args.get(0).unwrap().is_compatible(&operand) {
            return Err(ValidatorError {
                src: module.source(),
                at: item.span,
                err: "Operands are not compatible!".into(),
            }
            .into());
        }

        Ok(())
    }
}
