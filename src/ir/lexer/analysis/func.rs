use super::Analyzer;
use crate::{
    check_ir_token, IRFunction, IRNode, IRToken, IRTokenCursor, Result, Spanned, UnnamedLexerError,
};

impl Analyzer<IRFunction> for IRFunction {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRFunction>> {
        if item.0 == IRToken::Func {
            let it = cursor.next_or_die()?;

            let id = match it.0 {
                IRToken::Literal(it) => it,

                _ => {
                    return Err(UnnamedLexerError {
                        src: cursor.source(),
                        at: it.1,
                        err: format!("Unexpected token: {}", it.0),
                    }
                    .into())
                }
            };

            check_ir_token!(remove cursor == Colon);
            check_ir_token!(remove cursor == LeftBrace);

            let mut buf = Vec::new();
            let mut opens = 0;

            while let Some((tkn, span)) = cursor.next() {
                if tkn == IRToken::RightBrace {
                    if opens == 0 {
                        break;
                    } else {
                        opens -= 1;
                    }
                }

                if tkn == IRToken::LeftBrace {
                    opens += 1;
                }

                buf.push((tkn, span));
            }

            let mut body = Vec::new();
            let mut buf_cursor = IRTokenCursor::new_from_src(cursor.source(), buf);

            while let Some(item) = buf_cursor.next() {
                IRNode::analyze(item, &mut buf_cursor, &mut body)?;
            }

            Ok(Some(Self { id, body }))
        } else {
            Ok(None)
        }
    }
}
