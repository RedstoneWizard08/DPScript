use super::{Checker, CheckerContext};
use crate::{Conditional, Node, Result};

impl Checker<Conditional> for Conditional {
    fn check(item: &mut Conditional, cx: &mut CheckerContext) -> Result<()> {
        for item in &mut item.condition {
            Node::check(item, cx)?;
        }

        cx.cur_conds.push(item.clone());

        for item in &mut item.body {
            Node::check(item, cx)?;
        }

        cx.cur_conds.pop();
        cx.cur_elses.push(item.clone());

        for item in &mut item.else_body {
            Node::check(item, cx)?;
        }

        cx.cur_elses.pop();

        Ok(())
    }
}