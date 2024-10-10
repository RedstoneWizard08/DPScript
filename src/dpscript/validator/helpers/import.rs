use super::ExportNode;
use crate::{CheckerContext, ImportNode, Module, Result, ValidatorError};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ModuleImport {
    pub module: String,
    pub export: ExportNode,
}

impl Module {
    pub fn get_imported_objects(&mut self, cx: &CheckerContext) -> Result<Vec<ModuleImport>> {
        if let Some(objs) = &self.imported_objects {
            return Ok(objs.clone());
        }

        let mut objs = Vec::new();

        for import in self.imports() {
            let module = import.module();
            let module = cx.modules.get(&module);

            if let Some(module) = module {
                let exports = module.get_exports(cx)?;

                'item: for item in import.imports {
                    match item {
                        ImportNode::Object((obj, span)) => {
                            for export in &exports {
                                if export.name() == obj {
                                    objs.push(ModuleImport {
                                        module: module.name(),
                                        export: export.clone().node,
                                    });

                                    continue 'item;
                                }
                            }

                            return Err(ValidatorError {
                                src: self.source.clone(),
                                at: span,
                                err: format!("Could not resolve imported object: {}", obj),
                            }
                            .into());
                        }

                        _ => todo!("Nested imports"),
                    }
                }
            } else {
                return Err(ValidatorError {
                    src: self.source.clone(),
                    at: import.span,
                    err: format!("Could not resolve module: {}", import.module()),
                }
                .into());
            }
        }

        self.imported_objects = Some(objs.clone());

        Ok(objs)
    }

    /// Gets imported objects without re-caching.
    pub fn imported_objects(&self) -> Result<Vec<ModuleImport>> {
        if let Some(objs) = &self.imported_objects {
            Ok(objs.clone())
        } else {
            Err(ValidatorError {
                src: self.source.clone(),
                at: self.span,
                err: "Imported objects weren't cached yet! If you see this error, please create an issue on our GitHub!".into(),
            }.into())
        }
    }
}
