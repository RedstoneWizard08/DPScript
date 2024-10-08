use super::Analyzer;
use crate::{
    add_ir_return, IRArgumentOperation, IRBlock, IRCall, IRCommand, IRConcat, IRDataOperation,
    IRDefinition, IRExecute, IRFunction, IRLiteral, IRNode, IRParserError, IRTag, IRToken,
    IRTokenCursor, Result, Spanned,
};

impl Analyzer<IRNode> for IRNode {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRNode>> {
        if item.0 == IRToken::Semi {
            return Ok(None);
        }

        match IRDefinition::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Definition(v)),
            None => {}
        };

        match IRFunction::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Function(v)),
            None => {}
        };

        match IRConcat::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Concat(v)),
            None => {}
        };

        match IRArgumentOperation::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Argument(v)),
            None => {}
        };

        match IRTag::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Tag(v)),
            None => {}
        };

        match IRBlock::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Block(v)),
            None => {}
        };

        match IRCommand::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Command(v)),
            None => {}
        };

        match IRExecute::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Execute(v)),
            None => {}
        };

        match IRCall::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Call(v)),
            None => {}
        };

        match IRDataOperation::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += DataOperation(v)),
            None => {}
        };

        match IRLiteral::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_ir_return!(nodes += Literal(v)),
            None => {}
        };

        if let IRToken::Ident(id) = item.0 {
            Ok(Some(IRNode::Reference(id)))
        } else {
            Err(IRParserError {
                src: cursor.source(),
                at: item.1,
                err: format!("Unexpected token while parsing an IRNode: {}", item.0),
            }
            .into())
        }
    }
}
