use miette::SourceSpan;
use serde::Serialize;

use crate::{
    CheckerContext, Enum, Module, Objective, Result, Type, TypeKind, ValidatorError, Variable,
};

#[derive(Debug, Clone, Serialize)]
pub enum Reference {
    Variable(Variable),
    Objective(Objective),
    Enum(Enum),
}

impl Reference {
    pub fn name(&self) -> String {
        match self.clone() {
            Self::Variable(var) => var.name.0,
            Self::Objective(obj) => obj.name.0,
            Self::Enum(it) => it.name.0,
        }
    }

    pub fn span(&self) -> SourceSpan {
        match self.clone() {
            Self::Variable(var) => var.name.1,
            Self::Objective(obj) => obj.name.1,
            Self::Enum(it) => it.name.1,
        }
    }

    pub fn get_type(&self, module: &Module, cx: &mut CheckerContext) -> Result<Type> {
        match self.clone() {
            Self::Variable(var) => var.get_type(module, cx),
            Self::Objective(obj) => Ok(obj.get_type()),
            Self::Enum(it) => Err(ValidatorError {
                src: module.source.clone(),
                at: it.span,
                err: "Enums cannot be used as types!".into(),
            }
            .into()),
        }
    }

    pub fn is_array(&self, module: &Module, cx: &mut CheckerContext) -> Result<bool> {
        Ok(match self {
            Self::Variable(var) => match var.get_type(module, cx)?.kind {
                TypeKind::Array(_) => true,
                _ => false,
            },

            _ => false,
        })
    }
}

impl From<Variable> for Reference {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl From<Objective> for Reference {
    fn from(value: Objective) -> Self {
        Self::Objective(value)
    }
}

impl From<Enum> for Reference {
    fn from(value: Enum) -> Self {
        Self::Enum(value)
    }
}
