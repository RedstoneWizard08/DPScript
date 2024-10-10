use super::IRChecker;
use crate::{IRCheckerContext, IRLiteral, Result, UnsourcedValidatorError};

impl IRChecker for IRLiteral {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        match self {
            Self::PathOf(it) => {
                let refs = cx.get_refs();

                if let Some(it) = refs.get(it) {
                    *self = IRLiteral::String(it.path.clone());
                } else {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it),
                    }
                    .into());
                }

                Ok(())
            }

            Self::StoreOf(it) => {
                let refs = cx.get_refs();

                if let Some(it) = refs.get(it) {
                    *self = IRLiteral::String(it.store.clone());
                } else {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it),
                    }
                    .into());
                }

                Ok(())
            }

            Self::String(_) => Ok(()),
        }
    }
}
