use super::IRChecker;
use crate::{IRCheckerContext, IRExecute, Result, UnsourcedValidatorError};

impl IRChecker for IRExecute {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        let blocks = cx.get_blocks();

        if !blocks.contains_key(&self.block) {
            return Err(UnsourcedValidatorError {
                err: format!("Cannot find block: {}", self.block),
            }
            .into());
        }

        Ok(())
    }
}
