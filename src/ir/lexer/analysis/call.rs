use super::Analyzer;
use crate::{
    check_ir_token, IRCall, IRNode, IRParserError, IRToken, IRTokenCursor, Result, Spanned,
};

impl Analyzer<IRCall> for IRCall {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRCall>> {
        if item.0 == IRToken::Call {
            check_ir_token!(remove cursor == Colon);

            let it = cursor.next_or_die()?;

            let function = match it.0 {
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

            check_ir_token!(remove cursor == Semi);

            Ok(Some(Self { function }))
        } else {
            Ok(None)
        }
    }
}
