use super::Checker;
use crate::{CheckerContext, Conditional, Node, Result};

impl Checker<Conditional> for Conditional {
    fn check(item: &mut Conditional, cx: &mut CheckerContext) -> Result<()> {
        Node::check(&mut item.condition, cx)?;
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
