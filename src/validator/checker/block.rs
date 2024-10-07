use super::{Checker, CheckerContext};
use crate::{Block, Node, Result, ValidatorError};

impl Checker<Block> for Block {
    fn check(item: &mut Block, cx: &mut CheckerContext) -> Result<()> {
        if cx.cur_block.is_some() {
            return Err(ValidatorError {
                src: cx.get_source(),
                at: item.span,
                err: "Blocks cannot be nested!".into(),
            }
            .into());
        }

        cx.cur_block = Some(item.clone());

        for node in &mut item.body {
            Node::check(node, cx)?;
        }

        cx.cur_block = None;

        Ok(())
    }
}
