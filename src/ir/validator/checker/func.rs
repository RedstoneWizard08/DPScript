use super::IRChecker;
use crate::{IRCheckerContext, IRFunction, Result, UnsourcedValidatorError};

impl IRChecker for IRFunction {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        // TODO: Duplicate name checking

        if cx.cur_fn.is_some() {
            return Err(UnsourcedValidatorError {
                err: "Functions cannot be nested!".into(),
            }
            .into());
        }

        cx.cur_fn = Some(self.clone());

        for item in &mut self.body {
            item.check(cx)?;
        }

        cx.cur_fn = None;

        Ok(())
    }
}
