use super::Lowerable;
use crate::{
    AddDataOperation, CheckerContext, IRDataOperation, IRNode, LoweringContext, Result, Return,
};

impl Lowerable for Return {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        if let Some(val) = self.value.clone() {
            let set = IRNode::DataOperation(IRDataOperation::Set(AddDataOperation {
                var: "__RETURN_VAL__".into(),
                value: Box::new(val.get_value(cx, lcx, &mut nodes)?),
            }));

            nodes.push(set);
        }

        Ok(nodes)
    }
}
