use std::collections::BTreeMap;

use super::Lowerable;
use crate::{
    Block, CheckerContext, Function, IRBlock, IRFunction, IRNode, LoweringContext, Result,
};

impl Lowerable for Block {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.body.is_empty() {
            return Ok(Vec::new());
        }

        let name = if self.is_init { "init" } else { "tick" };
        let index = if self.is_init { lcx.inits } else { lcx.ticks };

        let id = format!(
            "{}:__dpscript_gen/{}/blocks/{}/{}",
            lcx.namespace, lcx.module, name, index
        );

        cx.cur_fn = Some(Function {
            name: (name.to_string(), self.span),
            args: Vec::new(),
            attrs: Vec::new(),
            ret: None,
            is_compiler: false,
            is_facade: false,
            is_inline: false,
            is_pub: false,
            body: self.body.clone(),
            span: self.span,
            locals: None,
        });

        cx.cur_fn.as_mut().unwrap().get_locals();

        lcx.blocks = 0;
        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();
        lcx.defs = BTreeMap::new();

        let mut items = Vec::new();
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

        nodes.push(IRNode::Function(IRFunction {
            id: id.clone(),
            body,
        }));

        nodes.extend(lcx.extra_nodes.clone());

        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();
        lcx.defs = BTreeMap::new();
        lcx.join_block = None;
        lcx.join_dirty = false;

        if self.is_init {
            lcx.init_names.push(id.clone());
        } else if self.is_tick {
            lcx.tick_names.push(id.clone());
        }

        Ok(nodes)
    }
}
