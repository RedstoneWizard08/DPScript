use super::Analyzer;
use crate::{
    check_ir_token, IRExecute, IRNode, IRParserError, IRToken, IRTokenCursor, Result, Spanned,
};

impl Analyzer<IRExecute> for IRExecute {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRExecute>> {
        if item.0 == IRToken::Execute {
            check_ir_token!(remove cursor == Colon);

            let it = cursor.next_or_die()?;

            let selector = match it.0 {
                IRToken::Literal(it) => it,

                _ => {
                    return Err(IRParserError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into())
                }
            };

            check_ir_token!(remove cursor == Comma);
            check_ir_token!(remove cursor == Dollar);

            let it = cursor.next_or_die()?;

            let block = match it.0 {
                IRToken::Ident(it) => it,

                _ => {
                    return Err(IRParserError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into())
                }
            };

            Ok(Some(Self { block, selector }))
        } else {
            Ok(None)
        }
    }
}
