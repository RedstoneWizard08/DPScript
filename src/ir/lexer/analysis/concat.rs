use super::Analyzer;
use crate::{IRConcat, IRNode, IRToken, IRTokenCursor, Result, Spanned};

impl Analyzer<IRConcat> for IRConcat {
    fn analyze(
        item: Spanned<IRToken>,
        cursor: &mut IRTokenCursor,
        _nodes: &mut Vec<IRNode>,
    ) -> Result<Option<IRConcat>> {
        if item.0 == IRToken::Plus || !cursor.peek().is_some_and(|(v, _)| v == IRToken::Plus) {
            return Ok(None);
        }

        let mut parts = Vec::new();
        let mut cur = Vec::new();

        cur.push(item);

        while let Some((tkn, span)) = cursor.next() {
            if tkn == IRToken::Plus {
                parts.push(cur);
                cur = Vec::new();
                continue;
            }

            cur.push((tkn, span));
        }

        if !cur.is_empty() {
            parts.push(cur);
        }

        let mut items = Vec::new();

        for part in parts {
            let mut part_cursor = IRTokenCursor::new_from_src(cursor.source(), part);

            while let Some(item) = part_cursor.next() {
                IRNode::analyze(item, &mut part_cursor, &mut items)?;
            }
        }

        Ok(Some(Self { items }))
    }
}
