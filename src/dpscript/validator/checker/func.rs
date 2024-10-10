use super::Checker;
use crate::{
    BuiltInEnums, BuiltInTypes, CheckerContext, Function, FunctionArg, Node, Result, ValidatorError,
};

impl Checker<Function> for Function {
    fn check(item: &mut Function, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.cur_modules.clone();
        let module = module.last().unwrap();

        if cx.cur_fn.is_some() {
            return Err(ValidatorError {
                src: module.source.clone(),
                at: item.span,
                err: "Functions cannot be nested!".into(),
            }
            .into());
        }

        cx.cur_fn = Some(item.clone());

        for arg in &mut item.args {
            FunctionArg::check(arg, cx)?;
        }

        if (item.is_compiler || item.is_facade) && !item.body.is_empty() {
            return Err(ValidatorError {
                src: module.source.clone(),
                at: item.name.1,
                err: format!("Facade or compiler builtin functions must not have a body!"),
            }
            .into());
        }

        for item in &mut item.body {
            Node::check(item, cx)?;
        }

        cx.cur_fn = None;

        Ok(())
    }
}

impl Checker<FunctionArg> for FunctionArg {
    fn check(item: &mut FunctionArg, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.cur_modules.last().unwrap();
        let mut ty = item.ty.kind.name();
        let imports = module.get_imported_names();

        let enums = module
            .get_enums()
            .iter()
            .map(|(v, _)| v.clone())
            .collect::<Vec<_>>();

        let mut all = BuiltInTypes::names()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

        all.extend(imports);
        all.extend(enums);

        all.extend(
            BuiltInEnums::names()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        );

        if ty.ends_with("[]") {
            ty = ty.trim_end_matches("[]").to_string();
        }

        if !all.contains(&ty) {
            return Err(ValidatorError {
                src: module.source.clone(),
                at: item.ty.span,
                err: format!("Could not resolve type: {}", item.ty.kind.name()),
            }
            .into());
        }

        Ok(())
    }
}
