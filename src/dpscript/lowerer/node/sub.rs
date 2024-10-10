use super::Lowerable;
use crate::{CheckerContext, IRBlock, IRNode, LoweringContext, Result, Subroutine};

impl Lowerable for Subroutine {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.body.is_empty() {
            return Ok(Vec::new());
        }

        let id = format!("__sub_{}", self.name.0);
        lcx.blocks += 1;

        let join_block = format!("block{}", lcx.blocks);
        lcx.blocks += 1;

        cx.cur_subroutines.push(self.clone());

        let mut body = Vec::new();
        let mut last_block: Option<IRBlock> = None;

        // This is kinda wonky but it works
        // TODO: Fix it
        for node in &mut self.body {
            let data = node.lower(cx, lcx)?;

            if let Some(block) = &mut lcx.join_block {
                if lcx.join_dirty {
                    if let Some(it) = &mut last_block {
                        it.body.extend(data);
                    } else {
                        body.extend(data);
                    }
                } else {
                    block.body.extend(data);
                }

                last_block = Some(block.clone());
            } else {
                body.extend(data);
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

        if let Some(block) = &mut lcx.join_block {
            block.body.push(IRNode::Goto(join_block.clone()));
        } else {
            body.push(IRNode::Goto(join_block.clone()));
        }

        lcx.block_nodes.push(IRBlock { id, body });

        lcx.join_block = Some(IRBlock {
            id: join_block,
            body: Vec::new(),
        });

        lcx.join_dirty = true;

        Ok(Vec::new())
    }
}
