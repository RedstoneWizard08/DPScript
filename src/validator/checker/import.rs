use super::{ctx::CheckerContext, Checker};
use crate::{Import, ImportNode, Module, Result, ValidatorError};

impl Checker<Import> for Import {
    fn check((module, _): &(String, Module), item: &mut Import, cx: &CheckerContext) -> Result<()> {
        let funcs = cx.funcs.get(&item.module());
        let vars = cx.vars.get(&item.module());
        let enums = cx.enums.get(&item.module());
        let objectives = cx.objectives.get(&item.module());

        for item in item.imports.clone() {
            match item {
                ImportNode::Object((name, span)) => {
                    let mut is_func = false;
                    let mut is_var = false;
                    let mut is_enum = false;
                    let mut is_objective = false;

                    if let Some(funcs) = funcs {
                        for func in funcs {
                            if func.name.0 == name && func.is_pub {
                                is_func = true;
                            }
                        }
                    }

                    if !is_func {
                        if let Some(vars) = vars {
                            for var in vars {
                                if var.name.0 == name && var.is_pub {
                                    is_var = true;
                                }
                            }
                        }
                    }

                    if !is_func && !is_var {
                        if let Some(enums) = enums {
                            for enum_ in enums {
                                if enum_.name.0 == name && enum_.is_pub {
                                    is_enum = true;
                                }
                            }
                        }
                    }

                    if !is_func && !is_var && !is_enum {
                        if let Some(objectives) = objectives {
                            for objective in objectives {
                                if objective.name.0 == name && objective.is_pub {
                                    is_objective = true;
                                }
                            }
                        }
                    }

                    if !is_func && !is_var && !is_enum && !is_objective {
                        let src = cx.modules.get(module).unwrap().source.clone();

                        return Err(ValidatorError {
                            src,
                            at: span,
                            err: format!("Identifier {} is not defined or is not public!", name),
                        }
                        .into());
                    }
                }

                _ => todo!("Implement Module objects (and use them somewhere)"),
            }
        }

        Ok(())
    }
}
