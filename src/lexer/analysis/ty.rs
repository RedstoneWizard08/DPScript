use crate::{ExpandSpan, Node, Result, Spanned, Token, TokenCursor, Type, TypeKind};

use super::Analyzer;

impl Analyzer<Type> for Type {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> Result<Option<Type>> {
        let id = match item.0 {
            Token::Ident(id) => (id, item.1),
            _ => return Ok(None),
        };

        let is_array = cursor
            .peek_ahead(1)
            .is_some_and(|(v, _)| v == Token::LeftBracket)
            && cursor
                .peek_ahead(2)
                .is_some_and(|(v, _)| v == Token::RightBracket);

        if is_array {
            cursor.skip(2);
        }

        let kind = match id.0.as_str() {
            "int" => TypeKind::Int,
            "float" => TypeKind::Float,
            "bool" => TypeKind::Bool,
            "str" | "String" => TypeKind::String,
            "NBT" => TypeKind::NBT,
            "NBTPath" => TypeKind::NBTPath,
            "Identifier" => TypeKind::Identifier,
            "Selector" => TypeKind::Selector,
            "Component" => TypeKind::Component,
            "Storage" => TypeKind::Storage,
            "Objective" => TypeKind::Objective,

            it => TypeKind::Ident(it.into()),
        };

        let ty = if is_array {
            Type {
                span: id.1.expand(2),
                kind: TypeKind::Array(Box::new(Type { kind, span: id.1 })),
            }
        } else {
            Type { kind, span: id.1 }
        };

        Ok(Some(ty))
    }
}
