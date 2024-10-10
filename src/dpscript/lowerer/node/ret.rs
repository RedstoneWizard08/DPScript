use super::{Lowerable, Valued};
use crate::{
    AddDataOperation, CheckerContext, CopyDataOperation, IRDataOperation, IRNode, LoweringContext,
    Result, Return,
};

impl Lowerable for Return {
    fn lower(&mut self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        if let Some(val) = &mut self.value {
            let val = val.get_value(cx, lcx, &mut nodes)?;

            if let IRNode::Reference(id) = &val {
                let copy = IRNode::DataOperation(IRDataOperation::Copy(CopyDataOperation {
                    source: id.clone(),
                    target: "__RETURN_VAL__".into(),
                }));

                nodes.push(copy);

                return Ok(nodes);
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
