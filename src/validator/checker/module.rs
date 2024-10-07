use super::{Checker, CheckerContext};
use crate::{Module, Node, Result};

impl Checker<Module> for Module {
    fn check(item: &mut Module, cx: &mut CheckerContext) -> Result<()> {
        item.get_imported_objects(cx)?;

        cx.cur_modules.push(item.clone());

        for node in &mut item.body {
            Node::check(node, cx)?;
        }

        cx.cur_modules.pop().unwrap();

        Ok(())
    }
}
