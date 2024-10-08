use super::{ctx::CheckerContext, Checker};
use crate::{
    Block, Call, Conditional, Enum, Function, Import, Loop, Node, Objective, Operation, Result,
    Return, ValidatorError, Variable,
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

            // Literals and idents on their own are always valid and exports are taken care of during `Module::get_exports()`
            Node::Export(_) | Node::Literal(_) | Node::Ident(_) => Ok(()),

            _ => Err(ValidatorError {
                src: module.source.clone(),
                at: item.span(),
                err: format!("Could not validate node: {}", item),
            }
            .into()),
        }
    }
}
