use super::IRChecker;
use crate::{IRCheckerContext, IRCommand, Result};

impl IRChecker for IRCommand {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        for item in &mut self.cmd {
            item.check(cx)?;
        }

        Ok(())
    }
}
