use crate::{
    AddSpan, LexerError, Node, Operation, OperationKind, Result, Spanned, Token, TokenCursor,
};

use super::Analyzer;

impl Analyzer<Operation> for Operation {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<Operation>> {
        let kind = match item.0 {
            Token::And => OperationKind::And,
            Token::Plus => OperationKind::Add,
            Token::Minus => OperationKind::Subtract,
            Token::Star => OperationKind::Multiply,
            Token::Slash => OperationKind::Divide,
            _ => return Ok(None),
        };

        if nodes.is_empty() {
            debug!("No nodes exist! How? Were we called to early?");

            return Ok(None);
        }

        let lhs = nodes.remove(nodes.len() - 1);
        let mut span = lhs.span();
        let tkn = cursor.next_or_die(item.1)?;
        let rhs = Node::analyze(tkn.clone(), cursor, &mut Vec::new())?;

        if let Some(rhs) = rhs {
            span = span.add(rhs.span());

            Ok(Some(Self {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                kind,
                span,
            }))
        } else {
            Err(LexerError {
                src: cursor.source(),
                at: tkn.1,
                err: format!("Could not parse an RHS operand: {}", tkn.0),
            }
            .into())
        }
    }
}
