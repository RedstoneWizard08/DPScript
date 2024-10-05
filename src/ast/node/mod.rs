pub mod attr;
pub mod block;
pub mod call;
pub mod cond;
pub mod enums;
pub mod export;
pub mod func;
pub mod import;
pub mod literal;
pub mod loops;
pub mod module;
pub mod objective;
pub mod ops;
pub mod ret;
pub mod ty;
pub mod var;

pub use attr::*;
pub use block::*;
pub use call::*;
pub use cond::*;
pub use enums::*;
pub use export::*;
pub use func::*;
pub use import::*;
pub use literal::*;
pub use loops::*;
use miette::SourceSpan;
pub use module::*;
pub use objective::*;
pub use ops::*;
pub use ret::*;
pub use ty::*;
pub use var::*;

use crate::Spanned;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Node {
    Module(Module),
    Literal(Literal),
    Ident(Spanned<String>),
    Import(Import),
    Function(Function),
    Variable(Variable),
    Call(Call),
    Operation(Operation),
    Block(Block),
    Loop(Loop),
    Enum(Enum),
    Return(Return),
    Objective(Objective),
    Conditional(Conditional),
    Export(Export),
}

#[derive(Debug, Clone, Serialize)]
pub enum TopLevelNode {
    Module(Module),
    Import(Import),
    Function(Function),
    Variable(Variable),
    Block(Block),
    Enum(Enum),
    Objective(Objective),
    Export(Export),
}

impl Node {
    pub fn get_span(&self) -> SourceSpan {
        match self {
            Self::Module(m) => m.span,
            Self::Literal(l) => l.get_span(),
            Self::Ident(i) => i.1,
            Self::Import(i) => i.span,
            Self::Function(f) => f.span,
            Self::Variable(v) => v.span,
            Self::Call(c) => c.span,
            Self::Operation(o) => o.span,
            Self::Block(b) => b.span,
            Self::Loop(l) => l.span,
            Self::Enum(e) => e.span,
            Self::Return(r) => r.span,
            Self::Objective(o) => o.span,
            Self::Conditional(c) => c.span,
            Self::Export(e) => e.span,
        }
    }
}

impl TopLevelNode {
    pub fn get_span(&self) -> SourceSpan {
        match self {
            Self::Module(m) => m.span,
            Self::Import(i) => i.span,
            Self::Function(f) => f.span,
            Self::Variable(v) => v.span,
            Self::Block(b) => b.span,
            Self::Enum(e) => e.span,
            Self::Objective(o) => o.span,
            Self::Export(e) => e.span,
        }
    }
}
