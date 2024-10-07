use super::ExportNode;
use crate::{BuiltInEnums, BuiltInTypes, Module, Result};
use miette::SourceSpan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AvailableType {
    /// The type's name
    pub name: String,

    /// This will be `None` if [`Self::is_builtin`] is `true`.
    pub span: Option<SourceSpan>,

    /// Is this a built-in type?
    pub is_builtin: bool,

    /// Is this an enum?
    pub is_enum: bool,

    /// This will be `None` if [`Self::is_enum`] is `false`.
    pub enum_entries: Option<Vec<String>>,
}

impl Module {
    pub fn available_types(&self) -> Result<Vec<AvailableType>> {
        let mut types = Vec::new();

        if let Ok(objs) = self.imported_objects() {
            for obj in objs {
                if let ExportNode::Enum(item) = obj.export {
                    types.push(AvailableType {
                        name: item.name.0.clone(),
                        span: Some(item.span),
                        enum_entries: Some(item.entries()),
                        is_builtin: false,
                        is_enum: true,
                    });
                }
            }
        }

        for name in BuiltInTypes::names() {
            types.push(AvailableType {
                name: name.into(),
                is_builtin: true,
                is_enum: false,
                span: None,
                enum_entries: None,
            });
        }

        for item in BuiltInEnums::all() {
            types.push(AvailableType {
                name: item.name().into(),
                is_builtin: true,
                is_enum: true,
                span: None,
                enum_entries: Some(item.variant_names().iter().map(|v| v.to_string()).collect()),
            });
        }

        Ok(types)
    }

    pub fn available_type_names(&self) -> Result<Vec<String>> {
        Ok(self
            .available_types()?
            .iter()
            .map(|v| v.name.clone())
            .collect())
    }
}
