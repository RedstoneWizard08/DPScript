use super::IRChecker;
use crate::{IRArgumentOperation, IRCheckerContext, IRNode, Result, UnsourcedValidatorError};

impl IRChecker for IRArgumentOperation {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        match self {
            Self::Set(it) => match *it.value.clone() {
                IRNode::Literal(_) => Ok(()),

                IRNode::Reference(it) => {
                    let refs = cx.get_refs();

                    if !refs.contains_key(&it) {
                        return Err(UnsourcedValidatorError {
                            err: format!("Cannot find reference: {}", it),
                        }
                        .into());
                    }

                    Ok(())
                }

                other => Err(UnsourcedValidatorError {
                    err: format!("Cannot set a {} as an argument value!", other),
                }
                .into()),
            },

            Self::Get(it) => {
                let refs = cx.get_refs();

                if !refs.contains_key(&it.var) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it.var),
                    }
                    .into());
                }

                Ok(())
            }

            Self::Clear => Ok(()),
        }
    }
}
