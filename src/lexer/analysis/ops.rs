use crate::{
    Node, Operation, OperationKind, ParserError, ParserResult, Spanned, Token, TokenCursor,
};

use super::Analyzer;

impl Analyzer<Operation> for Operation {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Operation>> {
        debug!("Trying to resolve an operation kind...");

        let kind = match item.0 {
            Token::And => OperationKind::And,
            Token::Plus => OperationKind::Add,
            Token::Minus => OperationKind::Subtract,
            Token::Star => OperationKind::Multiply,
            Token::Slash => OperationKind::Divide,
            _ => return Ok(None),
        };

        if nodes.is_empty() {
            debug!("No nodes exist! How?");

            return Ok(None);
        }

        debug!("Resolving the LHS operand...");

        let lhs = nodes.remove(nodes.len() - 1);
        let tkn = cursor.next_or_die(item.1)?;

        debug!("Resolving the RHS operand...");

        let rhs = Node::analyze(tkn.clone(), cursor, &mut Vec::new())?;

        if let Some(rhs) = rhs {
            Ok(Some(Self {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                kind,
            }))
        } else {
            Err(ParserError {
                src: cursor.source(),
                at: tkn.1,
                err: format!("Could not parse a right-hand side operation: {}", tkn.0),
            })
        }
    }
}
