use super::{Lowerable, Valued};
use crate::{
    AttributeValue, BuiltInTypes, Call, CheckerContext, ExportNode, Function, IRArgumentOperation,
    IRCall, IRCommand, IRLiteral, IRNode, IRSetArgument, LoweringContext, LoweringError, Node,
    Remap, Result, TypeKind,
};
use once_cell::sync::Lazy;
use regex::Regex;

pub const SELECTOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(?m)\{ "selector": "([^"]+)" \}"#).unwrap());

impl Call {
    fn handle_inline(
        &mut self,
        mut func: Function,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();
        let args = self.args.clone();
        let real_args = func.args.clone();

        for (i, mut arg) in args.iter().cloned().enumerate() {
            let real = real_args.get(i).unwrap();
            let value = arg.get_value(cx, lcx, &mut Vec::new())?;

            if let IRNode::Reference(id) = value {
                func = func.remap_name(&real.name.0, &id);
            }
        }

        for mut item in func.body {
            nodes.extend(item.lower(cx, lcx)?);
        }

        Ok(nodes)
    }

    fn handle_facade(
        &mut self,
        func: Function,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
    ) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        let Some(syntax) = func.attrs.iter().find(|v| v.name.0 == "cmd") else {
            return Err(LoweringError {
                src: cx.get_source(),
                at: func.span,
                err: "Facade function is missing #[cmd()] attribute!".into(),
            }
            .into());
        };

        let AttributeValue::String((it, _)) = syntax.value.clone() else {
            return Err(LoweringError {
                src: cx.get_source(),
                at: syntax.span,
                err: format!("Unexpected value: {:?}", syntax.value),
            }
            .into());
        };

        let parts = it
            .split_inclusive('%')
            .map(String::from)
            .collect::<Vec<_>>();

        let mut func_args = func.args.clone();
        let mut call_args = self.args.clone();
        let mut args = Vec::new();

        for mut item in parts {
            if item == "%" {
                let mut arg = call_args.remove(0);
                let real = func_args.remove(0);
                let value = arg.get_value(cx, lcx, &mut nodes)?;

                if real.ty.kind == TypeKind::Selector || real.ty.kind == TypeKind::Entity {
                    if let IRNode::Literal(lit) = value {
                        if let IRLiteral::String(val) = lit {
                            if SELECTOR_REGEX.is_match(&val) {
                                args.push(IRNode::Literal(
                                    SELECTOR_REGEX.replace(&val, "$1").into(),
                                ));

                                continue;
                            }
                        }
                    }
                }

                args.push(arg.get_value(cx, lcx, &mut nodes)?);
            } else {
                if item.ends_with('%') {
                    item = item.trim_end_matches('%').into();
                }

                if item.is_empty() || item.trim().is_empty() {
                    continue;
                }

                args.push(IRNode::Literal(item.trim().into()));
            }
        }

        nodes.push(IRCommand { cmd: args }.into());

        Ok(nodes)
    }
}

impl Lowerable for Call {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.function.0 == "storeof" {
            let id = match self.args.first().unwrap() {
                Node::Ident((id, _)) => id.clone(),

                it => {
                    return Err(LoweringError {
                        src: cx.get_source(),
                        at: it.span(),
                        err: "Expected an identifier!".into(),
                    }
                    .into())
                }
            };

            return Ok(vec![IRNode::Literal(IRLiteral::StoreOf(id))]);
        }

        if self.function.0 == "keyof" {
            let id = match self.args.first().unwrap() {
                Node::Ident((id, _)) => id.clone(),

                it => {
                    return Err(LoweringError {
                        src: cx.get_source(),
                        at: it.span(),
                        err: "Expected an identifier!".into(),
                    }
                    .into())
                }
            };

            return Ok(vec![IRNode::Literal(IRLiteral::PathOf(id))]);
        }

        let mut nodes = Vec::new();
        let mut args = Vec::new();

        if let Some((parent, _)) = self.parent.clone() {
            args.push(IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                index: 0,
                value: Box::new(IRNode::Reference(parent)),
            })));
        }

        for arg in &mut self.args {
            args.push(IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                index: args.len(),
                value: Box::new(arg.get_value(cx, lcx, &mut nodes)?),
            })));
        }

        let mut module = cx.modules.get(&lcx.module).unwrap().clone();
        let refs = cx.get_refs()?;
        let objs = module.get_imported_objects(cx)?;
        let mut found = false;

        if let Some((name, span)) = self.parent.clone() {
            let mut parent = None;

            for (item, it) in refs {
                if name == item {
                    parent = Some(it);
                    break;
                }
            }

            if parent.is_none() {
                return Err(LoweringError {
                    src: module.source(),
                    at: span,
                    err: format!("Cannot resolve object: {}", name),
                }
                .into());
            }

            let parent = parent.unwrap();
            let base_ty = BuiltInTypes::from(parent.get_type(&module, cx)?);
            let methods = base_ty.methods();
            let mut method = None;

            for (name, args, ret) in methods {
                if name == self.function.0 {
                    method = Some((name, args, ret));
                    break;
                }
            }

            if method.is_none() {
                return Err(LoweringError {
                    src: module.source(),
                    at: self.function.1,
                    err: format!(
                        "Object {} does not define a method: {}",
                        name, self.function.0
                    ),
                }
                .into());
            }

            let (id, _, _) = method.unwrap();

            nodes.extend(args);

            nodes.push(IRNode::Call(IRCall {
                function: format!(
                    "{}:__dpscript_gen/core/{}/{}",
                    lcx.namespace,
                    base_ty.name(),
                    id
                ),
            }));

            return Ok(nodes);
        }

        for item in objs {
            if let ExportNode::Function(func) = item.export {
                if func.name.0 == self.function.0 {
                    if func.is_facade {
                        nodes.extend(self.handle_facade(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    if func.is_inline {
                        nodes.extend(self.handle_inline(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    let id = func.ir_name(&lcx.namespace, item.module);

                    nodes.push(IRNode::Call(IRCall { function: id }));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for func in module.funcs() {
                if func.name.0 == self.function.0 {
                    if func.is_facade {
                        nodes.extend(self.handle_facade(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    if func.is_inline {
                        nodes.extend(self.handle_inline(func, cx, lcx)?);
                        found = true;
                        break;
                    }

                    nodes.extend(args);

                    let id = func.ir_name(&lcx.namespace, &lcx.module);

                    nodes.push(IRNode::Call(IRCall { function: id }));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Err(LoweringError {
                src: cx.get_source(),
                at: self.span,
                err: format!("Could not find function: {}", self.function.0),
            }
            .into());
        }

        Ok(nodes)
    }
}
