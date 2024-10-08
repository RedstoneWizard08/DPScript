use super::Lowerable;
use crate::{
    Call, CheckerContext, ExportNode, IRArgumentOperation, IRCall, IRNode, IRSetArgument,
    LoweringContext, Result,
};

impl Lowerable for Call {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();
        let mut args = Vec::new();

        for (i, arg) in self.args.iter().enumerate() {
            args.push(IRNode::Argument(IRArgumentOperation::Set(IRSetArgument {
                index: i,
                value: Box::new(arg.get_value(cx, lcx, &mut nodes)?),
            })));
        }

        nodes.extend(args);

        let module = cx.modules.get(&lcx.module).unwrap();
        let objs = module.imported_objects()?;
        let mut found = false;

        for item in objs {
            if let ExportNode::Function(func) = item.export {
                if func.name.0 == self.function.0 {
                    let id = func.ir_name(&lcx.namespace, item.module);

                    nodes.push(IRNode::Call(IRCall { function: id }));
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for func in module.funcs() {
                if func.name.0 == self.function.0 {
                    let id = func.ir_name(&lcx.namespace, &lcx.module);

                    nodes.push(IRNode::Call(IRCall { function: id }));
                    break;
                }
            }
        }

        Ok(nodes)
    }
}
