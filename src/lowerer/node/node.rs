use super::Lowerable;
use crate::{CheckerContext, IRNode, IRParserError, LoweringContext, Node, Result};

impl Node {
    pub fn get_value(
        &self,
        cx: &mut CheckerContext,
        lcx: &mut LoweringContext,
        nodes: &mut Vec<IRNode>,
    ) -> Result<IRNode> {
        match self {
            Self::Literal(lit) => lit.get_value(cx, nodes),
            Self::Ident((id, _)) => Ok(IRNode::Reference(id.clone())),

            Self::Call(call) => {
                nodes.extend(call.lower(cx, lcx)?);

                Ok(IRNode::Reference("__RETURN_VAL__".into()))
            }

            Self::Operation(op) => {
                nodes.extend(op.lower(cx, lcx)?);

                Ok(IRNode::Reference("__RETURN_VAL__".into()))
            }

            _ => Err(IRParserError {
                src: cx.get_source().inner().to_owned(),
                at: self.span(),
                err: "This cannot be used as a value!".into(),
            }
            .into()),
        }
    }
}

impl Lowerable for Node {
    fn lower(&self, cx: &mut CheckerContext, lcx: &mut LoweringContext) -> Result<Vec<IRNode>> {
        let mut nodes = Vec::new();

        nodes.extend(match self.clone() {
            Self::Module(it) => it.lower(cx, lcx)?,
            Self::Literal(it) => it.lower(cx, lcx)?,
            Self::Function(it) => it.lower(cx, lcx)?,
            Self::Variable(it) => it.lower(cx, lcx)?,
            Self::Call(it) => it.lower(cx, lcx)?,
            Self::Operation(it) => it.lower(cx, lcx)?,
            Self::Block(it) => it.lower(cx, lcx)?,
            Self::Loop(it) => it.lower(cx, lcx)?,
            Self::Return(it) => it.lower(cx, lcx)?,
            Self::Objective(it) => it.lower(cx, lcx)?,
            Self::Conditional(it) => it.lower(cx, lcx)?,

            // Imports, exports, types, and enums don't get transpiled - they're types.
            _ => Vec::new(),
        });

        Ok(nodes)
    }
}
