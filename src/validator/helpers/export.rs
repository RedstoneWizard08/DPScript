use miette::SourceSpan;
use serde::Serialize;

use crate::{
    CheckerContext, Enum, Export, Function, ImportNode, Module, Objective, Result, ValidatorError,
    Variable,
};

#[derive(Debug, Clone, Serialize)]
pub struct ModuleExport {
    pub module: String,
    pub node: ExportNode,
}

#[derive(Debug, Clone, Serialize)]
pub enum ExportNode {
    Module(Module),
    Function(Function),
    Variable(Variable),
    Enum(Enum),
    Objective(Objective),
}

macro_rules! publicity_error {
    ($ty: expr => $src: ident, $item: ident, $($n: tt)+) => {
        return Err(ValidatorError {
            src: $src,
            at: $item.span,
            err: format!(
                "Exported {} \"{}\" is not public!",
                stringify!($ty),
                $($n)+
            ),
        }
        .into());
    };
}

impl Export {
    pub fn get_exports(&self, module: &Module, cx: &CheckerContext) -> Result<Vec<ModuleExport>> {
        let mut nodes = Vec::new();
        let exported_module = format!("{}/{}", module.name(), self.module_name());

        if let Some(module) = cx.modules.get(&exported_module) {
            let mut mod_nodes = Vec::new();

            if self.all {
                for export in module.exports() {
                    nodes.extend(export.get_exports(module, cx)?);
                }

                for module in module.modules() {
                    if module.is_pub {
                        mod_nodes.push(ExportNode::Module(module));
                    }
                }

                for func in module.funcs() {
                    if func.is_pub {
                        mod_nodes.push(ExportNode::Function(func));
                    }
                }

                for var in module.vars() {
                    if var.is_pub {
                        mod_nodes.push(ExportNode::Variable(var));
                    }
                }

                for item in module.enums() {
                    if item.is_pub {
                        mod_nodes.push(ExportNode::Enum(item));
                    }
                }

                for obj in module.objectives() {
                    if obj.is_pub {
                        mod_nodes.push(ExportNode::Objective(obj));
                    }
                }
            } else {
                'item: for item in &self.items {
                    match item {
                        ImportNode::Object((obj, span)) => {
                            let obj = obj.to_owned();
                            let src = module.source.clone();

                            for module in module.modules() {
                                if module.name() == obj {
                                    if module.is_pub {
                                        mod_nodes.push(ExportNode::Module(module));
                                        continue 'item;
                                    } else {
                                        publicity_error!("module" => src, module, module.name());
                                    }
                                }
                            }

                            for func in module.funcs() {
                                if func.name.0 == obj {
                                    if func.is_pub {
                                        mod_nodes.push(ExportNode::Function(func));
                                        continue 'item;
                                    } else {
                                        publicity_error!("function" => src, func, func.name.0);
                                    }
                                }
                            }

                            for var in module.vars() {
                                if var.name.0 == obj {
                                    if var.is_pub {
                                        mod_nodes.push(ExportNode::Variable(var));
                                        continue 'item;
                                    } else {
                                        publicity_error!("variable" => src, var, var.name.0);
                                    }
                                }
                            }

                            for item in module.enums() {
                                if item.name.0 == obj {
                                    if item.is_pub {
                                        mod_nodes.push(ExportNode::Enum(item));
                                        continue 'item;
                                    } else {
                                        publicity_error!("enum" => src, item, item.name.0);
                                    }
                                }
                            }

                            for item in module.objectives() {
                                if item.name.0 == obj {
                                    if item.is_pub {
                                        mod_nodes.push(ExportNode::Objective(item));
                                        continue 'item;
                                    } else {
                                        publicity_error!("objective" => src, item, item.name.0);
                                    }
                                }
                            }

                            return Err(ValidatorError {
                                src: module.source.clone(),
                                at: span.to_owned(),
                                err: format!("Exported object \"{}\" is undefined!", obj),
                            }
                            .into());
                        }

                        _ => todo!("Nested imports"),
                    }
                }
            }

            let name = module.name();

            nodes.extend(mod_nodes.iter().map(|node| ModuleExport {
                module: name.clone(),
                node: node.to_owned(),
            }));
        }

        Ok(nodes)
    }
}

impl ModuleExport {
    pub fn name(&self) -> String {
        self.node.name()
    }

    pub fn span(&self) -> SourceSpan {
        self.node.span()
    }
}

impl ExportNode {
    pub fn name(&self) -> String {
        match self.clone() {
            Self::Enum(item) => item.name.0,
            Self::Function(item) => item.name.0,
            Self::Module(item) => item.name(),
            Self::Objective(item) => item.name.0,
            Self::Variable(item) => item.name.0,
        }
    }

    pub fn span(&self) -> SourceSpan {
        match self.clone() {
            Self::Enum(item) => item.span,
            Self::Function(item) => item.span,
            Self::Module(item) => item.span,
            Self::Objective(item) => item.span,
            Self::Variable(item) => item.span,
        }
    }
}
