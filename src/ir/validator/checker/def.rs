use super::IRChecker;
use crate::{IRCheckerContext, IRDefinition, Result};

impl IRChecker for IRDefinition {
    fn check(&mut self, _cx: &mut IRCheckerContext) -> Result<()> {
        // TODO: Check for duplicate names

        Ok(())
    }
}
