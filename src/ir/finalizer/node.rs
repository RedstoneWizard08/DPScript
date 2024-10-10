use crate::{
    IRArgumentOperation, IRCheckerContext, IRCommand, IRDataOperation, IRFunction, IRNode, Result,
    UnsourcedValidatorError,
};

pub const ARG_STORE: &str = "dpscript:core/args";

impl IRNode {
    pub fn finalize(&mut self, cx: &mut IRCheckerContext, extra: &mut Vec<IRNode>) -> Result<()> {
        let refs = cx.get_refs();

        match self {
            Self::Block(it) => {
                let id = format!("{}/blocks/{}", cx.cur_fn.clone().unwrap().id, it.id);

                cx.cur_block = Some(it.clone());

                for item in &mut it.body {
                    item.finalize(cx, extra)?;
                }

                cx.cur_block = None;

                extra.push(IRNode::Function(IRFunction {
                    id,
                    body: it.body.clone(),
                }));

                *self = IRNode::None;

                Ok(())
            }

            Self::Function(it) => {
                cx.cur_fn = Some(it.clone());

                for item in &mut it.body {
                    item.finalize(cx, extra)?;
                }

                cx.cur_fn = None;

                Ok(())
            }

            Self::Call(it) => {
                *self = IRNode::Command(IRCommand {
                    cmd: vec![
                        IRNode::Literal("function".into()),
                        IRNode::Literal(it.function.clone().to_lowercase().into()),
                    ],
                });

                Ok(())
            }

            Self::Goto(it) => {
                *self = IRNode::Command(IRCommand {
                    cmd: vec![
                        IRNode::Literal("function".into()),
                        IRNode::Literal(
                            format!("{}/blocks/{}", cx.cur_fn.clone().unwrap().id, it)
                                .to_lowercase()
                                .into(),
                        ),
                    ],
                });

                Ok(())
            }

            Self::Condition(it) => {
                *self = IRNode::Group(vec![
                    IRNode::Command(IRCommand {
                        cmd: vec![
                            IRNode::Literal("execute if score __dpscript_temp".into()),
                            IRNode::Literal(it.score.clone().into()),
                            IRNode::Literal("matches 1 run function".into()),
                            IRNode::Literal(
                                format!("{}/blocks/{}", cx.cur_fn.clone().unwrap().id, it.if_block)
                                    .to_lowercase()
                                    .into(),
                            ),
                        ],
                    }),
                    IRNode::Command(IRCommand {
                        cmd: vec![
                            IRNode::Literal("execute unless score __dpscript_temp".into()),
                            IRNode::Literal(it.score.clone().into()),
                            IRNode::Literal("matches 1 run function".into()),
                            IRNode::Literal(
                                format!(
                                    "{}/blocks/{}",
                                    cx.cur_fn.clone().unwrap().id,
                                    it.else_block
                                )
                                .to_lowercase()
                                .into(),
                            ),
                        ],
                    }),
                ]);

                Ok(())
            }

            Self::Execute(it) => {
                *self = IRNode::Command(IRCommand {
                    cmd: vec![
                        IRNode::Literal("execute as".into()),
                        IRNode::Literal(it.selector.clone().into()),
                        IRNode::Literal("run function".into()),
                        IRNode::Literal(
                            format!("{}/blocks/{}", cx.cur_fn.clone().unwrap().id, it.block)
                                .to_lowercase()
                                .into(),
                        ),
                    ],
                });

                Ok(())
            }

            Self::Argument(it) => match it {
                IRArgumentOperation::Set(it) => match *it.value.clone() {
                    IRNode::Literal(lit) => {
                        *self = IRNode::Command(IRCommand {
                            cmd: vec![
                                IRNode::Literal("data modify storage".into()),
                                IRNode::Literal(ARG_STORE.into()),
                                IRNode::Literal(format!("__arg_{}", it.index).into()),
                                IRNode::Literal("set value".into()),
                                IRNode::Literal(lit),
                            ],
                        });

                        Ok(())
                    }

                    IRNode::Reference(id) => {
                        let Some(var) = refs.get(&id) else {
                            return Err(UnsourcedValidatorError {
                                err: format!("Cannot resolve reference: {}", id),
                            }
                            .into());
                        };

                        *self = IRNode::Command(IRCommand {
                            cmd: vec![
                                IRNode::Literal("data modify storage".into()),
                                IRNode::Literal(ARG_STORE.into()),
                                IRNode::Literal(format!("__arg_{}", it.index).into()),
                                IRNode::Literal("set from storage".into()),
                                IRNode::Literal(var.store.clone().into()),
                                IRNode::Literal(var.path.clone().into()),
                            ],
                        });

                        Ok(())
                    }

                    other => Err(UnsourcedValidatorError {
                        err: format!("Cannot set variable to a value of a {}!", other),
                    }
                    .into()),
                },

                IRArgumentOperation::Get(it) => {
                    let Some(var) = refs.get(&it.var) else {
                        return Err(UnsourcedValidatorError {
                            err: format!("Cannot resolve reference: {}", it.var),
                        }
                        .into());
                    };

                    *self = IRNode::Command(IRCommand {
                        cmd: vec![
                            IRNode::Literal("data modify storage".into()),
                            IRNode::Literal(var.store.clone().into()),
                            IRNode::Literal(var.path.clone().into()),
                            IRNode::Literal("set from storage".into()),
                            IRNode::Literal(ARG_STORE.into()),
                            IRNode::Literal(format!("__arg_{}", it.index).into()),
                        ],
                    });

                    Ok(())
                }

                IRArgumentOperation::Clear => {
                    *self = IRNode::Command(IRCommand {
                        cmd: vec![
                            IRNode::Literal("data merge storage".into()),
                            IRNode::Literal(ARG_STORE.into()),
                            IRNode::Literal("{}".into()),
                        ],
                    });

                    Ok(())
                }
            },

            Self::DataOperation(it) => match it {
                IRDataOperation::Set(it) => {
                    let Some(out) = refs.get(&it.var) else {
                        return Err(UnsourcedValidatorError {
                            err: format!("Cannot resolve reference: {}", it.var),
                        }
                        .into());
                    };

                    match *it.value.clone() {
                        IRNode::Literal(it) => {
                            *self = IRNode::Command(IRCommand {
                                cmd: vec![
                                    IRNode::Literal("data modify storage".into()),
                                    IRNode::Literal(out.store.clone().into()),
                                    IRNode::Literal(out.path.clone().into()),
                                    IRNode::Literal("set value".into()),
                                    IRNode::Literal(it),
                                ],
                            });

                            Ok(())
                        }

                        IRNode::Reference(id) => {
                            let Some(var) = refs.get(&id) else {
                                return Err(UnsourcedValidatorError {
                                    err: format!("Cannot resolve reference: {}", id),
                                }
                                .into());
                            };

                            *self = IRNode::Command(IRCommand {
                                cmd: vec![
                                    IRNode::Literal("data modify storage".into()),
                                    IRNode::Literal(out.store.clone().into()),
                                    IRNode::Literal(out.path.clone().into()),
                                    IRNode::Literal("set from storage".into()),
                                    IRNode::Literal(var.store.clone().into()),
                                    IRNode::Literal(var.path.clone().into()),
                                ],
                            });

                            Ok(())
                        }

                        other => Err(UnsourcedValidatorError {
                            err: format!("Cannot set variable to a value of a {}!", other),
                        }
                        .into()),
                    }
                }

                IRDataOperation::Copy(it) => {
                    let Some(out) = refs.get(&it.target) else {
                        return Err(UnsourcedValidatorError {
                            err: format!("Cannot resolve reference: {}", it.target),
                        }
                        .into());
                    };

                    let Some(var) = refs.get(&it.target) else {
                        return Err(UnsourcedValidatorError {
                            err: format!("Cannot resolve reference: {}", it.target),
                        }
                        .into());
                    };

                    *self = IRNode::Command(IRCommand {
                        cmd: vec![
                            IRNode::Literal("data modify storage".into()),
                            IRNode::Literal(out.store.clone().into()),
                            IRNode::Literal(out.path.clone().into()),
                            IRNode::Literal("set from storage".into()),
                            IRNode::Literal(var.store.clone().into()),
                            IRNode::Literal(var.path.clone().into()),
                        ],
                    });

                    Ok(())
                }

                IRDataOperation::Append(it) => {
                    let Some(out) = refs.get(&it.var) else {
                        return Err(UnsourcedValidatorError {
                            err: format!("Cannot resolve reference: {}", it.var),
                        }
                        .into());
                    };

                    match *it.value.clone() {
                        IRNode::Literal(it) => {
                            *self = IRNode::Command(IRCommand {
                                cmd: vec![
                                    IRNode::Literal("data modify storage".into()),
                                    IRNode::Literal(out.store.clone().into()),
                                    IRNode::Literal(out.path.clone().into()),
                                    IRNode::Literal("append value".into()),
                                    IRNode::Literal(it),
                                ],
                            });

                            Ok(())
                        }

                        IRNode::Reference(id) => {
                            let Some(var) = refs.get(&id) else {
                                return Err(UnsourcedValidatorError {
                                    err: format!("Cannot resolve reference: {}", id),
                                }
                                .into());
                            };

                            *self = IRNode::Command(IRCommand {
                                cmd: vec![
                                    IRNode::Literal("data modify storage".into()),
                                    IRNode::Literal(out.store.clone().into()),
                                    IRNode::Literal(out.path.clone().into()),
                                    IRNode::Literal("append from storage".into()),
                                    IRNode::Literal(var.store.clone().into()),
                                    IRNode::Literal(var.path.clone().into()),
                                ],
                            });

                            Ok(())
                        }

                        other => Err(UnsourcedValidatorError {
                            err: format!("Cannot set variable to a value of a {}!", other),
                        }
                        .into()),
                    }
                }
            },

            _ => Ok(()),
        }
    }
}
