use super::Lowerable;
use crate::{Block, CheckerContext, Function, IRFunction, IRNode, LoweringContext, Result};

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
            body: Vec::new(),
            span: self.span,
            locals: None,
        });

        cx.cur_fn.as_mut().unwrap().get_locals();

        lcx.blocks = 0;
        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();

        let mut body = Vec::new();

        for item in &mut self.body {
            body.extend(item.lower(cx, lcx)?);
        }

        if self.is_init {
            lcx.init_names.push(id.clone());
        } else if self.is_tick {
            lcx.tick_names.push(id.clone());
        }

        cx.cur_fn = None;
        lcx.blocks = 0;

        body.extend(lcx.block_nodes.iter().map(|v| IRNode::Block(v.clone())));

        let mut nodes = Vec::new();

        nodes.push(IRNode::Function(IRFunction { id, body }));
        nodes.extend(lcx.extra_nodes.clone());

        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();

        Ok(nodes)
    }
}
