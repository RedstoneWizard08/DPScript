use super::{Checker, CheckerContext};
use crate::{Node, Result, Subroutine};

impl Checker<Subroutine> for Subroutine {
    fn check(item: &mut Subroutine, cx: &mut CheckerContext) -> Result<()> {
        // TODO: Check for name duplication

        cx.cur_subroutines.push(item.clone());

        for node in &mut item.body {
            Node::check(node, cx)?;
        }

        cx.cur_subroutines.pop();

        Ok(())
    }
}
