use super::Lowerable;
use crate::{
    CheckerContext, IRArgumentOperation, IRBlock, IRDefinition, IRExecute, IRGetArgument, IRNode,
    Literal, Loop, LoweringContext, LoweringError, Node, Reference, Result, TypeKind,
    VariableAlias,
};

impl Lowerable for Loop {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.body.is_empty() {
            return Ok(Vec::new());
        }

        let mut nodes = Vec::new();
        let refs = cx.get_refs()?;

        let Some(it) = refs.get(&self.array.0) else {
            return Err(LoweringError {
                src: cx.get_source(),
                at: self.array.1,
                err: format!("Could not resolve reference: {}", self.array.0),
            }
            .into());
        };

        cx.cur_loops.push(self.clone());

        match it {
            Reference::Variable(var) => {
                let ty = var.get_type(&cx.current_module(), cx)?;
                let mut body = Vec::new();

                if ty.kind == TypeKind::Selector {
                    for item in &mut self.body {
                        body.extend(item.lower(cx, lcx)?);
                    }

                    let id = format!("block{}", lcx.blocks);

                    lcx.blocks += 1;

                    lcx.block_nodes.push(IRBlock {
                        id: id.clone(),
                        body,
                    });

                    nodes.push(IRNode::Execute(IRExecute {
                        block: id,
                        selector: match *var.value.clone().unwrap() {
                            Node::Literal(lit) => match lit {
                                Literal::Selector((sel, _)) => sel,

                                _ => {
                                    return Err(LoweringError {
                                        src: cx.get_source(),
                                        at: lit.span(),
                                        err: "Could not get a value out of a literal!".into(),
                                    }
                                    .into())
                                }
                            },

                            it => {
                                return Err(LoweringError {
                                    src: cx.get_source(),
                                    at: it.span(),
                                    err: "Could not get a value out of a node!".into(),
                                }
                                .into())
                            }
                        },
                    }));
                } else {
                    let var = if self.var_name.0.starts_with("__var_")
                        || self.var_name.0 == "__RETURN_VAL__"
                    {
                        self.var_name.0.clone()
                    } else {
                        format!("__var_{}", self.var_name.0)
                    };

                    body.push(IRNode::Definition(IRDefinition::VariableAlias(
                        VariableAlias {
                            name: var.clone(),
                            store: "dpscript:core/vars".into(),
                            path: var.clone(),
                        },
                    )));

                    body.push(IRNode::Argument(IRArgumentOperation::Get(IRGetArgument {
                        index: 0,
                        var: var.clone(),
                    })));

                    for item in &mut self.body {
                        body.extend(item.lower(cx, lcx)?);
                    }

                    todo!("For each loops")
                }
            }

            _ => panic!("[Achievement Unlocked] How did we get here?"),
        }

        cx.cur_loops.pop();

        Ok(nodes)
    }
}