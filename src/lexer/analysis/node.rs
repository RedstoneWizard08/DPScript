use crate::{
    add_return, Block, Call, Conditional, Enum, Export, Function, Import, Literal, Loop, Module,
    Node, Objective, Operation, ParserError, Result, Return, Spanned, Token, TokenCursor, Variable,
};

use super::Analyzer;

impl Analyzer<Node> for Node {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> Result<Option<Node>> {
        if item.0 == Token::Semi {
            return Ok(None);
        }

        match Module::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Module(v)),
            None => {}
        };

        match Import::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Import(v)),
            None => {}
        };

        match Export::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Export(v)),
            None => {}
        };

        match Variable::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Variable(v)),
            None => {}
        };

        match Block::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Block(v)),
            None => {}
        };

        match Function::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Function(v)),
            None => {}
        };

        match Loop::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Loop(v)),
            None => {}
        };

        match Conditional::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Conditional(v)),
            None => {}
        };

        match Enum::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Enum(v)),
            None => {}
        };

        match Literal::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Literal(v)),
            None => {}
        };

        match Call::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Call(v)),
            None => {}
        };

        match Operation::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Operation(v)),
            None => {}
        };

        match Return::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Return(v)),
            None => {}
        };

        match Objective::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Objective(v)),
            None => {}
        };

        match String::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Ident(v)),
            None => {}
        }

        Err(ParserError {
            src: cursor.source(),
            at: item.1,
            err: format!("Unexpected token while parsing a node: {}", item.0),
        }
        .into())
    }
}
