use super::IRChecker;
use crate::{IRBlock, IRCheckerContext, Result, UnsourcedValidatorError};

impl IRChecker for IRBlock {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        // TODO: Duplicate name checking

        if cx.cur_block.is_some() {
            return Err(UnsourcedValidatorError {
                err: "Blocks cannot be nested!".into(),
            }
            .into());
        }

        cx.cur_block = Some(self.clone());

        for item in &mut self.body {
            item.check(cx)?;
        }

        cx.cur_block = None;

        Ok(())
    }
}
