use super::IRChecker;
use crate::{IRCheckerContext, IRDataOperation, IRNode, Result, UnsourcedValidatorError};

impl IRChecker for IRDataOperation {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        match self {
            Self::Append(it) => {
                let refs = cx.get_refs();

                if !refs.contains_key(&it.var) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it.var),
                    }
                    .into());
                }

                match *it.value.clone() {
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
                }
            }

            Self::Copy(it) => {
                let refs = cx.get_refs();

                if !refs.contains_key(&it.source) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it.source),
                    }
                    .into());
                }

                if !refs.contains_key(&it.target) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it.target),
                    }
                    .into());
                }

                Ok(())
            }

            Self::Set(it) => {
                let refs = cx.get_refs();

                if !refs.contains_key(&it.var) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it.var),
                    }
                    .into());
                }

                match *it.value.clone() {
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
                }
            }
        }
    }
}
