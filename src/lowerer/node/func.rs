use super::Lowerable;
use crate::{
    CheckerContext, Function, IRArgumentOperation, IRDefinition, IRFunction, IRGetArgument, IRNode,
    LoweringContext, Result, VariableAlias,
};

impl Lowerable for Function {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.is_compiler {
            return Ok(vec![]);
        }

        // TODO: Facade functions

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

        for node in &self.body {
            body.extend(node.lower(cx, lcx)?);
        }

        Ok(vec![IRNode::Function(IRFunction { id, body })])
    }
}
