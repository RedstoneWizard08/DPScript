use super::Lowerable;
use crate::{
    CheckerContext, IRArgumentOperation, IRNode, IRSetArgument, LoweringContext, Operation, Result,
};

impl Lowerable for Operation {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        let lhs = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
            index: 0,
            value: Box::new(self.lhs.get_value(cx, lcx, &mut nodes)?),
        }));

        let rhs = IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
            index: 0,
            value: Box::new(self.rhs.get_value(cx, lcx, &mut nodes)?),
        }));

        nodes.push(lhs);
        nodes.push(rhs);

        // TODO: Get the IR function name and call it

        Ok(nodes)
    }
}
