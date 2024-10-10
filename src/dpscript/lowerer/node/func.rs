use std::collections::BTreeMap;

use super::Lowerable;
use crate::{
    CheckerContext, Function, IRArgumentOperation, IRBlock, IRDefinition, IRFunction,
    IRGetArgument, IRNode, InsertHelper, LoweringContext, Result, VariableAlias,
};

impl Lowerable for Function {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.is_compiler || self.is_inline || self.is_facade {
            return Ok(vec![]);
        }

        if self.body.is_empty() {
            return Ok(Vec::new());
        }

        cx.cur_fn = Some(self.clone());
        lcx.blocks = 0;
        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();
        lcx.defs = BTreeMap::new();

        let id = self.ir_name(&lcx.namespace, &lcx.module);
        let mut items = Vec::new();

        for (i, arg) in self.args.iter().enumerate() {
            let name = format!("__var_{}", arg.name.0);

            lcx.defs.insert_if_absent(
                name.clone(),
                IRDefinition::VariableAlias(VariableAlias {
                    name,
                    path: format!("__var_{}", arg.name.0),
                    store: format!("{}:__dpscript/vars/local", lcx.namespace),
                }),
            );

            items.push(IRNode::Argument(IRArgumentOperation::Get(IRGetArgument {
                index: i,
                var: format!("__var_{}", arg.name.0),
            })));
        }

        let mut last_block: Option<IRBlock> = None;

        for node in &mut self.body {
            let data = node.lower(cx, lcx)?;

            if let Some(block) = &mut lcx.join_block {
                if lcx.join_dirty {
                    if let Some(it) = &mut last_block {
                        it.body.extend(data);
                    } else {
                        items.extend(data);
                    }
                } else {
                    block.body.extend(data);
                }

                last_block = Some(block.clone());
            } else {
                items.extend(data);
            }

            if lcx.join_dirty {
                if let Some(block) = last_block {
                    lcx.block_nodes.push(block);
                }

                last_block = None;
                lcx.join_dirty = false;
            }
        }

        if let Some(block) = lcx.join_block.clone() {
            if lcx.block_nodes.iter().find(|v| v.id == block.id).is_none() {
                lcx.block_nodes.push(block);
            }
        }

        cx.cur_fn = None;
        lcx.blocks = 0;

        let mut body = Vec::new();

        body.extend(lcx.defs.iter().map(|(_, v)| IRNode::Definition(v.clone())));
        body.extend(items);
        body.extend(lcx.block_nodes.iter().map(|v| IRNode::Block(v.clone())));

        let mut nodes = Vec::new();

        nodes.push(IRNode::Function(IRFunction { id, body }));
        nodes.extend(lcx.extra_nodes.clone());

        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();
        lcx.defs = BTreeMap::new();
        lcx.join_block = None;
        lcx.join_dirty = false;

        Ok(nodes)
    }
}
