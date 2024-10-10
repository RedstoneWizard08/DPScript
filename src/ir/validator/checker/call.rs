use super::IRChecker;
use crate::{IRCall, IRCheckerContext, Result, UnsourcedValidatorError};

impl IRChecker for IRCall {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        if !cx.funcs.contains_key(&self.function) {
            return Err(UnsourcedValidatorError {
                err: format!("Cannot find function: {}", self.function),
            }
            .into());
        }

        Ok(())
    }
}
