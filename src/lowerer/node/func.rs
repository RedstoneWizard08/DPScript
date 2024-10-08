use super::Lowerable;
use crate::{
    CheckerContext, Function, IRArgumentOperation, IRDefinition, IRFunction, IRGetArgument, IRNode,
    LoweringContext, Result, VariableAlias,
};

impl Lowerable for Function {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.is_compiler || self.is_inline || self.is_facade {
            return Ok(vec![]);
        }

        if self.body.is_empty() {
            return Ok(Vec::new());
        }

        // TODO: Facade functions

        cx.cur_fn = Some(self.clone());
        lcx.blocks = 0;
        lcx.extra_nodes = Vec::new();
        lcx.block_nodes = Vec::new();

        let id = self.ir_name(&lcx.namespace, &lcx.module);
        let mut body = Vec::new();

        for (i, arg) in self.args.iter().enumerate() {
            body.push(IRNode::Definition(IRDefinition::VariableAlias(
                VariableAlias {
                    name: format!("arg_{}", arg.name.0),
                    path: format!("{}:__dpscript/vars/local", lcx.namespace),
                    store: format!("__arg_{}_{}", arg.name.0, i),
                },
            )));

            body.push(IRNode::Argument(IRArgumentOperation::Get(IRGetArgument {
                index: i,
                var: format!("arg_{}", arg.name.0),
            })));
        }

        for node in &mut self.body {
            body.extend(node.lower(cx, lcx)?);
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
