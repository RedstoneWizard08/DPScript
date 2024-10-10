use super::IRChecker;
use crate::{IRCheckerContext, IRCondition, Result, UnsourcedValidatorError};

impl IRChecker for IRCondition {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        let blocks = cx.get_blocks();

        if !blocks.contains_key(&self.if_block) {
            return Err(UnsourcedValidatorError {
                err: format!("Cannot find block: {}", self.if_block),
            }
            .into());
        }

        if !blocks.contains_key(&self.else_block) {
            return Err(UnsourcedValidatorError {
                err: format!("Cannot find block: {}", self.else_block),
            }
            .into());
        }

        if !blocks.contains_key(&self.join_block) {
            return Err(UnsourcedValidatorError {
                err: format!("Cannot find block: {}", self.join_block),
            }
            .into());
        }

        Ok(())
    }
}
