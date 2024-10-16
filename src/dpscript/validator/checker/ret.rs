use super::Checker;
use crate::{CheckerContext, Node, Result, Return};

impl Checker<Return> for Return {
    fn check(item: &mut Return, cx: &mut CheckerContext) -> Result<()> {
        if let Some(val) = &mut item.value {
            Node::check(val, cx)?;
        }

        Ok(())
    }
}
