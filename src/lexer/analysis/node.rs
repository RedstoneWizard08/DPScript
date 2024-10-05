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

        debug!("Trying to parse an export...");

        match Export::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Export(v)),
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

        debug!("Trying to parse a conditional...");

        match Conditional::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Conditional(v)),
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

        debug!("Trying to parse a return...");

        match Return::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Return(v)),
            None => {}
        };

        debug!("Trying to parse an objective...");

        match Objective::analyze(item.clone(), cursor, nodes)? {
            Some(v) => add_return!(nodes += Objective(v)),
            None => {}
        };

        debug!("Trying to parse an identifier...");

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
