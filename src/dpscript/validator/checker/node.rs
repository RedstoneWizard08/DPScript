use super::Checker;
use crate::{
    Block, Call, CheckerContext, Conditional, Enum, Function, Import, Loop, Node, Objective,
    Operation, Result, Return, Subroutine, ValidatorError, Variable,
};

impl Checker<Node> for Node {
    fn check(item: &mut Node, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.cur_modules.clone();
        let module = module.last().unwrap();

        match item {
            Node::Block(block) => Block::check(block, cx),
            Node::Import(import) => Import::check(import, cx),
            Node::Objective(obj) => Objective::check(obj, cx),
            Node::Function(func) => Function::check(func, cx),
            Node::Enum(enum_) => Enum::check(enum_, cx),
            Node::Variable(var) => Variable::check(var, cx),
            Node::Call(call) => Call::check(call, cx),
            Node::Conditional(cond) => Conditional::check(cond, cx),
            Node::Return(ret) => Return::check(ret, cx),
            Node::Loop(it) => Loop::check(it, cx),
            Node::Operation(op) => Operation::check(op, cx),
            Node::Subroutine(sub) => Subroutine::check(sub, cx),

            Node::Ident(id) => {
                let refs = cx.get_refs()?;
                let subs = cx.get_subroutines();

                if !refs.contains_key(&id.0) && !subs.contains_key(&id.0) {
                    return Err(ValidatorError {
                        src: module.source.clone(),
                        at: id.1,
                        err: format!("Could not resolve reference: {}", id.0),
                    }
                    .into());
                }

                Ok(())
            }

            Node::Goto(it) => {
                if !cx.get_subroutines().contains_key(&it.0) {
                    return Err(ValidatorError {
                        src: module.source.clone(),
                        at: it.1,
                        err: format!("Could not find subroutine: {}", it.0),
                    }
                    .into());
                }

                Ok(())
            }

            // Literals on their own are always valid and exports are taken care of during `Module::get_exports()`
            Node::Export(_) | Node::Literal(_) => Ok(()),

            _ => Err(ValidatorError {
                src: module.source.clone(),
                at: item.span(),
                err: format!("Could not validate node: {}", item),
            }
            .into()),
        }
    }
}
