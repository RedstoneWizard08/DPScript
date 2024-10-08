use super::{Lowerable, Valued};
use crate::{
    AddDataOperation, CheckerContext, IRDataOperation, IRDefinition, IRNode, LoweringContext,
    Result, Variable, VariableAlias,
};

impl Lowerable for Variable {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.is_const {
            return Ok(Vec::new());
        }

        let mut nodes = Vec::new();
        let var = format!(
            "__var_{}",
            if self.name.0.starts_with("__var_") || self.name.0 == "__RETURN_VAL__" {
                self.name.0.clone()
            } else {
                format!("__var_{}", self.name.0)
            }
        );

        nodes.push(IRNode::Definition(IRDefinition::VariableAlias(
            VariableAlias {
                name: var.clone(),
                store: "dpscript:core/vars".into(),
                path: var.clone(),
            },
        )));

        if let Some(val) = &mut self.value {
            let node = IRNode::DataOperation(IRDataOperation::Set(AddDataOperation {
                var,
                value: Box::new(val.get_value(cx, lcx, &mut nodes)?),
            }));

            nodes.push(node);
        }

        Ok(nodes)
    }
}
