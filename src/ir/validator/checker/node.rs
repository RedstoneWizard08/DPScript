use crate::{IRCheckerContext, IRNode, Result, UnsourcedValidatorError};

use super::IRChecker;

impl IRChecker for IRNode {
    fn check(&mut self, cx: &mut IRCheckerContext) -> Result<()> {
        match self {
            Self::Definition(it) => it.check(cx),
            Self::DataOperation(it) => it.check(cx),
            Self::Function(it) => it.check(cx),
            Self::Concat(it) => it.check(cx),
            Self::Literal(it) => it.check(cx),
            Self::Argument(it) => it.check(cx),
            Self::Tag(it) => it.check(cx),
            Self::Block(it) => it.check(cx),
            Self::Command(it) => it.check(cx),
            Self::Execute(it) => it.check(cx),
            Self::Call(it) => it.check(cx),
            Self::Condition(it) => it.check(cx),
            Self::Group(_) | Self::None => Ok(()),

            Self::Reference(it) => {
                let refs = cx.get_refs();

                if !refs.contains_key(it) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find reference: {}", it),
                    }
                    .into());
                }

                Ok(())
            }

            Self::Goto(it) => {
                let blocks = cx.get_blocks();

                if !blocks.contains_key(it) {
                    return Err(UnsourcedValidatorError {
                        err: format!("Cannot find block: {}", it),
                    }
                    .into());
                }

                Ok(())
            }
        }
    }
}
