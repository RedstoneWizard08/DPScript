use super::IRChecker;
use crate::{IRCheckerContext, IRTag, Result, UnsourcedValidatorError};

impl IRChecker for IRTag {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        for item in &self.entries {
            if !cx.funcs.contains_key(item) {
                return Err(UnsourcedValidatorError {
                    err: format!("Cannot find function: {}", item),
                }
                .into());
            }
        }

        Ok(())
    }
}
