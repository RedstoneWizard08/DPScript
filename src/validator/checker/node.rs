use super::{ctx::CheckerContext, Checker};
use crate::{Import, Module, Node, Result, ValidatorError};

impl Checker<Node> for Node {
    fn check(module: &(String, Module), item: &mut Node, cx: &CheckerContext) -> Result<()> {
        match item {
            Node::Import(import) => Import::check(module, import, cx),
            Node::Export(_) => Ok(()),
            Node::Literal(_) => Ok(()),

            Node::Ident(_) => Err(ValidatorError {
                src: module.1.source.clone(),
                at: item.get_span(),
                err: format!("No context for identifier!"),
            }
            .into()),

            // Module(Module),
            // Function(Function),
            // Variable(Variable),
            // Call(Call),
            // Operation(Operation),
            // Block(Block),
            // Loop(Loop),
            // Enum(Enum),
            // Return(Return),
            // Objective(Objective),
            // Conditional(Conditional),
            _ => Err(ValidatorError {
                src: module.1.source.clone(),
                at: item.get_span(),
                err: format!("Could not validate node: {}", item),
            }
            .into()),
        }
    }
}
