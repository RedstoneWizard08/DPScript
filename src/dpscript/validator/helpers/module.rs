use super::{ExportNode, ModuleExport};
use crate::{CheckerContext, Module, Result};

impl Module {
    pub fn get_exports(&self, cx: &CheckerContext) -> Result<Vec<ModuleExport>> {
        let module = self.name();
        let mut exports = Vec::new();

        for node in self.exports() {
            exports.extend(node.get_exports(self, cx)?);
        }

        for item in self.modules() {
            if item.is_pub {
                exports.push(ModuleExport {
                    module: module.clone(),
                    node: ExportNode::Module(item),
                });
            }
        }

        for item in self.funcs() {
            if item.is_pub {
                exports.push(ModuleExport {
                    module: module.clone(),
                    node: ExportNode::Function(item),
                });
            }
        }

        for item in self.vars() {
            if item.is_pub {
                exports.push(ModuleExport {
                    module: module.clone(),
                    node: ExportNode::Variable(item),
                });
            }
        }

        for item in self.enums() {
            if item.is_pub {
                exports.push(ModuleExport {
                    module: module.clone(),
                    node: ExportNode::Enum(item),
                });
            }
        }

        for item in self.objectives() {
            if item.is_pub {
                exports.push(ModuleExport {
                    module: module.clone(),
                    node: ExportNode::Objective(item),
                });
            }
        }

        Ok(exports)
    }
}
