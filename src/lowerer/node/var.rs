use super::{Lowerable, Valued};
use crate::{
    AddDataOperation, CheckerContext, CopyDataOperation, IRDataOperation, IRDefinition, IRNode,
    InsertHelper, LoweringContext, Result, Variable, VariableAlias,
};

impl Lowerable for Variable {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        if self.is_const {
            return Ok(Vec::new());
        }

        let mut nodes = Vec::new();

        let var = if self.name.0.starts_with("__var_") || self.name.0 == "__RETURN_VAL__" {
            self.name.0.clone()
        } else {
            format!("__var_{}", self.name.0)
        };

        lcx.defs.insert_if_absent(
            var.clone(),
            IRDefinition::VariableAlias(VariableAlias {
                name: var.clone(),
                store: "dpscript:core/vars".into(),
                path: var.clone(),
            }),
        );

        if let Some(val) = &mut self.value {
            let val = val.get_value(cx, lcx, &mut nodes)?;

            if let IRNode::Reference(it) = val {
                let node = IRNode::DataOperation(IRDataOperation::Copy(CopyDataOperation {
                    source: it,
                    target: var,
                }));

                nodes.push(node);
            } else {
                let node = IRNode::DataOperation(IRDataOperation::Set(AddDataOperation {
                    var,
                    value: Box::new(val),
                }));

                nodes.push(node);
            }
        }

        Ok(nodes)
    }
}
