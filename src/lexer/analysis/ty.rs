use crate::{ExpandSpan, Node, ParserResult, Spanned, Token, TokenCursor, Type, TypeKind};

use super::Analyzer;

impl Analyzer<Type> for Type {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        _nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Type>> {
        let id = match item.0 {
            Token::Ident(id) => (id, item.1),
            _ => return Ok(None),
        };

        let is_array = cursor.peek().is_some_and(|(v, _)| v == Token::LeftBracket)
            && cursor
                .peek_ahead(1)
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

        let mut ty = Type { kind, span: id.1 };

        if is_array {
            ty = Type {
                span: ty.span.expand(2),
                kind: TypeKind::Array(Box::new(ty)),
            };
        }

        Ok(Some(ty))
    }
}
