use super::IRChecker;
use crate::{IRCheckerContext, IRConcat, Result};

impl IRChecker for IRConcat {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        for item in &mut self.items {
            item.check(cx)?;
        }

        Ok(())
    }
}
