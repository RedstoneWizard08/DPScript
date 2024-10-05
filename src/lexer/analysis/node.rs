use crate::{
    add_return, Block, Call, Enum, Function, Import, Literal, Loop, Module, Node, Operation,
    ParserError, ParserResult, Spanned, Token, TokenCursor, Variable,
};

use super::Analyzer;

impl Analyzer<Node> for Node {
    fn analyze(
        item: Spanned<Token>,
        cursor: &mut TokenCursor,
        nodes: &mut Vec<Node>,
    ) -> ParserResult<Option<Node>> {
        if item.0 == Token::Semi {
            return Ok(None);
        }

        debug!("Trying to parse a module...");

        match Module::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Module(v)),
            None => {}
        };

        debug!("Trying to parse an import...");

        match Import::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Import(v)),
            None => {}
        };

        debug!("Trying to parse a variable...");

        match Variable::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Variable(v)),
            None => {}
        };

        debug!("Trying to parse a block...");

        match Block::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Block(v)),
            None => {}
        };

        debug!("Trying to parse a function...");

        match Function::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Function(v)),
            None => {}
        };

        debug!("Trying to parse a loop...");

        match Loop::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Loop(v)),
            None => {}
        };

        debug!("Trying to parse an enum...");

        match Enum::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Enum(v)),
            None => {}
        };

        debug!("Trying to parse a literal...");

        match Literal::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Literal(v)),
            None => {}
        };

        debug!("Trying to parse a call...");

        match Call::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Call(v)),
            None => {}
        };

        debug!("Trying to parse an operation...");

        match Operation::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Operation(v)),
            None => {}
        };

        debug!("Trying to parse an identifier...");

        match String::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Ident(v)),
            None => {}
        }

        // TODO: conditionals, enum value, return

        Err(ParserError {
            src: cursor.source(),
            at: item.1,
            err: format!("Unexpected token while parsing a node: {}", item.0),
        })
    }
}
