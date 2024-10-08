use super::{Lowerable, Valued};
use crate::{
    AddDataOperation, CheckerContext, IRDataOperation, IRNode, LoweringContext, Result, Return,
};

impl Lowerable for Return {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        if let Some(val) = &mut self.value {
            let val = val.get_value(cx, lcx, &mut nodes)?;

            if let IRNode::Reference(id) = &val {
                if id == "__RETURN_VAL__" {
                    return Ok(Vec::new());
                }
            }

            let set = IRNode::DataOperation(IRDataOperation::Set(AddDataOperation {
                var: "__RETURN_VAL__".into(),
                value: Box::new(val),
            }));

            nodes.push(set);
        }

        Ok(nodes)
    }
}
