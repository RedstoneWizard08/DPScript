use std::collections::HashMap;

use serde_json::Value;

use super::{ctx::CheckerContext, Checker};
use crate::{
    Block, Call, Conditional, Enum, Function, Import, Literal, Loop, Node, Objective, Operation,
    Result, Return, ValidatorError, Variable,
};

impl Checker<Node> for Node {
    fn check(item: &mut Node, cx: &mut CheckerContext) -> Result<()> {
        let module = cx.cur_modules.clone();
        let module = module.last().unwrap();

        match item {
            Node::Block(block) => Block::check(block, cx),
            Node::Import(import) => Import::check(import, cx),
            Node::Objective(obj) => Objective::check(obj, cx),
            Node::Function(func) => Function::check(func, cx),
            Node::Enum(enum_) => Enum::check(enum_, cx),
            Node::Variable(var) => Variable::check(var, cx),
            Node::Call(call) => Call::check(call, cx),
            Node::Conditional(cond) => Conditional::check(cond, cx),
            Node::Return(ret) => Return::check(ret, cx),
            Node::Loop(it) => Loop::check(it, cx),

            Node::Operation(op) => {
                if let Node::Literal(lhs) = &*op.lhs {
                    if let Node::Literal(rhs) = &*op.rhs {
                        if let Literal::Component((comp, _)) = lhs {
                            if let Literal::Nbt((nbt, _)) = rhs {
                                let mut map = serde_json::from_str::<HashMap<String, Value>>(
                                    &serde_json::to_string(&nbt)?,
                                )?;

                                map.insert("text".into(), Value::String(comp.into()));

                                *item = Node::Literal(Literal::Nbt((
                                    serde_json::from_str(&serde_json::to_string(&map)?)?,
                                    op.span,
                                )));

                                return Ok(());
                            }
                        } else if let Literal::Nbt((nbt, _)) = lhs {
                            if let Literal::Component((comp, _)) = rhs {
                                let mut map = serde_json::from_str::<HashMap<String, Value>>(
                                    &serde_json::to_string(&nbt)?,
                                )?;

                                map.insert("text".into(), Value::String(comp.into()));

                                *item = Node::Literal(Literal::Nbt((
                                    serde_json::from_str(&serde_json::to_string(&map)?)?,
                                    op.span,
                                )));

                                return Ok(());
                            }
                        } else if let Literal::Entity((ent, _)) = lhs {
                            if let Literal::Nbt((nbt, _)) = rhs {
                                let mut map = serde_json::from_str::<HashMap<String, Value>>(
                                    &serde_json::to_string(&nbt)?,
                                )?;

                                map.insert("selector".into(), Value::String(ent.into()));

                                *item = Node::Literal(Literal::Nbt((
                                    serde_json::from_str(&serde_json::to_string(&map)?)?,
                                    op.span,
                                )));

                                return Ok(());
                            }
                        } else if let Literal::Nbt((nbt, _)) = lhs {
                            if let Literal::Entity((ent, _)) = rhs {
                                let mut map = serde_json::from_str::<HashMap<String, Value>>(
                                    &serde_json::to_string(&nbt)?,
                                )?;

                                map.insert("selector".into(), Value::String(ent.into()));

                                *item = Node::Literal(Literal::Nbt((
                                    serde_json::from_str(&serde_json::to_string(&map)?)?,
                                    op.span,
                                )));

                                return Ok(());
                            }
                        } else if let Literal::Selector((sel, _)) = lhs {
                            if let Literal::Nbt((nbt, _)) = rhs {
                                let mut map = serde_json::from_str::<HashMap<String, Value>>(
                                    &serde_json::to_string(&nbt)?,
                                )?;

                                map.insert("selector".into(), Value::String(sel.into()));

                                *item = Node::Literal(Literal::Nbt((
                                    serde_json::from_str(&serde_json::to_string(&map)?)?,
                                    op.span,
                                )));

                                return Ok(());
                            }
                        } else if let Literal::Nbt((nbt, _)) = lhs {
                            if let Literal::Selector((sel, _)) = rhs {
                                let mut map = serde_json::from_str::<HashMap<String, Value>>(
                                    &serde_json::to_string(&nbt)?,
                                )?;

                                map.insert("selector".into(), Value::String(sel.into()));

                                *item = Node::Literal(Literal::Nbt((
                                    serde_json::from_str(&serde_json::to_string(&map)?)?,
                                    op.span,
                                )));

                                return Ok(());
                            }
                        }
                    }
                }

                Operation::check(op, cx)
            }

            // Literals and idents on their own are always valid and exports are taken care of during `Module::get_exports()`
            Node::Export(_) | Node::Literal(_) | Node::Ident(_) => Ok(()),

            _ => Err(ValidatorError {
                src: module.source.clone(),
                at: item.span(),
                err: format!("Could not validate node: {}", item),
            }
            .into()),
        }
    }
}
