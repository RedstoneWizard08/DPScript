pub mod call;
pub mod component;
pub mod function;
pub mod nbt;
pub mod operation;
pub mod selector;
pub mod variable;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    compiler::Compilable, error::CompilationError, lines::LineBuilder, source, state::State,
    Result, DPSCRIPT_RETURN_VAR, DPSCRIPT_TEMP_STORE, DPSCRIPT_VAR_STORE,
};

use self::{
    call::Call,
    component::Component,
    function::Func,
    nbt::{Nbt, NbtItem},
    operation::Operation,
    selector::Selector,
    variable::Var,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Int(i32),
    Float(f32),
    Ident(String),
    String(String),
    Component(Component),
    Selector(Selector),
    Nbt(Nbt),
    Import(String),
    Call(Call),
    Command(Call),
    Func(Func),
    Var(Var),
    Operation(Operation),
    Array(Vec<Expr>),
    Return(Box<Expr>),

    None,
}

impl Expr {
    pub fn get_type(&self, state: &State) -> Result<String> {
        match self.clone() {
            Self::Int(_) => Ok("Int".into()),
            Self::Float(_) => Ok("Float".into()),
            Self::String(_) => Ok("String".into()),
            Self::Component(_) => Ok("Component".into()),
            Self::Selector(_) => Ok("Entity".into()),
            Self::Nbt(_) => Ok("Nbt".into()),
            Self::Operation(op) => op.get_type(state),

            Self::Array(arr) => {
                let ty = arr.first().unwrap().get_type(state)?;

                if !arr.iter().all(|v| v.get_type(state).unwrap() == ty) {
                    return Err(CompilationError {
                        src: source!(state),
                        err: format!("Array elements must all be of the same type!"),
                    }
                    .into());
                }

                Ok(ty)
            }

            Self::Call(call) => {
                if let Some(func) = state.functions.get(&call.func) {
                    Ok(func.1.ret.clone().unwrap_or("Void".into()))
                } else {
                    Err(CompilationError {
                        src: source!(state),
                        err: format!("Cannot find a function named {}!", call.func),
                    }
                    .into())
                }
            }

            Self::Ident(id) => {
                if let Some(var) = state.locals.get(&id) {
                    Ok(var.1.ty.clone().unwrap_or(var.1.value.get_type(state)?))
                } else if let Some(var) = state.globals.get(&id) {
                    Ok(var.1.ty.clone().unwrap_or(var.1.value.get_type(state)?))
                } else {
                    Err(CompilationError {
                        src: source!(state),
                        err: format!("Cannot find a variable named {}!", id),
                    }
                    .into())
                }
            }

            _ => Ok("Void".into()),
        }
    }

    pub fn as_component(&self, state: &State) -> Result<Component> {
        match self.clone() {
            Self::Ident(v) => {
                if let Some(var) = state.locals.get(&v) {
                    Ok(Component::from_map(HashMap::from_iter([
                        ("storage".into(), NbtItem::String(DPSCRIPT_VAR_STORE.into())),
                        ("nbt".into(), NbtItem::String(var.0.clone())),
                        ("interpret".into(), NbtItem::Bool(true)),
                    ])))
                } else if let Some(var) = state.globals.get(&v) {
                    Ok(Component::from_map(HashMap::from_iter([
                        ("storage".into(), NbtItem::String(DPSCRIPT_VAR_STORE.into())),
                        ("nbt".into(), NbtItem::String(var.0.clone())),
                        ("interpret".into(), NbtItem::Bool(true)),
                    ])))
                } else {
                    Err(CompilationError {
                        src: source!(state),
                        err: format!("Cannot find a variable named {}!", v),
                    }
                    .into())
                }
            }

            Self::Float(v) => Ok(Component::new(v.to_string())),
            Self::Int(v) => Ok(Component::new(v.to_string())),
            Self::String(v) => Ok(Component::new(v)),

            v => Err(CompilationError {
                err: format!("Cannot convert Expr {:?} into a component!", v),
                src: source!(state),
            }
            .into()),
        }
    }

    pub fn fix(&self) -> Self {
        match self.clone() {
            Expr::Call(mut call) => {
                call.args = call
                    .args
                    .iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect();

                Expr::Call(call)
            }

            Expr::Command(mut call) => {
                call.args = call
                    .args
                    .iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect();

                Expr::Command(call)
            }

            Expr::Func(mut func) => {
                func.body = func
                    .body
                    .iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect();

                Expr::Func(func)
            }

            Expr::Var(mut var) => {
                var.value = Box::new(var.value.fix());

                Expr::Var(var)
            }

            Expr::Component(mut comp) => {
                if let Some(expr) = &mut comp.from_expr {
                    *expr = Box::new(expr.fix());
                }

                Expr::Component(comp)
            }

            Expr::Operation(mut op) => {
                op.lhs = Box::new(op.lhs.fix());
                op.rhs = Box::new(op.rhs.fix());

                Expr::Operation(op)
            }

            Expr::Array(arr) => Expr::Array(
                arr.iter()
                    .cloned()
                    .filter(|v| v.clone() != Expr::None)
                    .collect(),
            ),

            Expr::Return(expr) => Expr::Return(Box::new(expr.fix())),

            v => v,
        }
    }

    /// Determines whether or not this will require a `set value` in data commands.
    pub fn is_value(&self) -> bool {
        match self.clone() {
            Expr::Float(_)
            | Expr::Int(_)
            | Expr::String(_)
            | Expr::Selector(_)
            | Expr::Component(_)
            | Expr::Nbt(_) => true,

            _ => false,
        }
    }

    /// Determines whether or not this will require a `set from [store] [path]`
    /// in data commands.
    pub fn is_ref(&self) -> bool {
        !self.is_value()
    }

    pub fn compile(&self, state: &mut State, id: impl AsRef<str>) -> Result<String> {
        match self.clone() {
            Expr::Float(v) => Ok(v.to_string()),
            Expr::Int(v) => Ok(v.to_string()),
            Expr::String(v) => Ok(format!("\"{}\"", v)),

            Expr::Ident(v) => {
                if let Some((n, _)) = state.locals.get(&v) {
                    Ok(format!(
                        "data modify storage {} {} set from storage {} {}",
                        DPSCRIPT_TEMP_STORE,
                        id.as_ref(),
                        DPSCRIPT_VAR_STORE,
                        n
                    ))
                } else if let Some((_, v)) = state.globals.clone().get(&v) {
                    let temp = "__global_ref_temp";
                    let mut b = LineBuilder::new();

                    b.push(v.value.compile(state, temp)?);

                    b.push(format!(
                        "data modify storage {} {} set from storage {} {}",
                        DPSCRIPT_TEMP_STORE,
                        id.as_ref(),
                        DPSCRIPT_TEMP_STORE,
                        temp,
                    ));

                    Ok(b.build())
                } else {
                    Err(CompilationError {
                        src: source!(state),
                        err: format!("Cannot find a variable named {}!", v),
                    }
                    .into())
                }
            }

            Expr::Call(call) => {
                let mut out = LineBuilder::from_str(call.compile(state)?);

                out.push(format!(
                    "data modify storage {} {} set from storage {} {}",
                    DPSCRIPT_TEMP_STORE,
                    id.as_ref(),
                    DPSCRIPT_TEMP_STORE,
                    DPSCRIPT_RETURN_VAR
                ));

                Ok(out.build())
            }

            Expr::Command(call) => {
                let mut out = LineBuilder::from_str(call.compile(state)?);

                out.push(format!(
                    "data modify storage {} {} set from storage {} {}",
                    DPSCRIPT_TEMP_STORE,
                    id.as_ref(),
                    DPSCRIPT_TEMP_STORE,
                    DPSCRIPT_RETURN_VAR
                ));

                Ok(out.build())
            }

            Expr::Array(arr) => {
                let mut b = LineBuilder::new();

                b.push(format!(
                    "data modify storage {} {} set value []",
                    DPSCRIPT_TEMP_STORE,
                    id.as_ref()
                ));

                for (i, item) in arr.iter().enumerate() {
                    if item.is_value() {
                        b.push(format!(
                            "data modify storage {} {} append value {}",
                            DPSCRIPT_TEMP_STORE,
                            id.as_ref(),
                            item.compile(state, "")?
                        ));
                    } else {
                        let temp = format!("__array_item_{}", i);

                        b.push(item.compile(state, id.as_ref())?);

                        b.push(format!(
                            "data modify storage {} {} append from storage {} {}",
                            DPSCRIPT_TEMP_STORE,
                            id.as_ref(),
                            DPSCRIPT_TEMP_STORE,
                            temp
                        ));
                    }
                }

                Ok(b.build())
            }

            // Imports are syntax sugar, so we ignore them here.
            Expr::Import(_) => Ok(String::new()),

            Expr::Return(ret) => {
                if ret.is_value() {
                    Ok(format!(
                        "data modify storage {} {} set value {}",
                        DPSCRIPT_TEMP_STORE,
                        DPSCRIPT_RETURN_VAR,
                        ret.compile(state, "")?
                    ))
                } else {
                    let temp = "__return_data";
                    let mut b = LineBuilder::new();

                    b.push(ret.compile(state, temp)?);

                    b.push(format!(
                        "data modify storage {} {} set from storage {} {}",
                        DPSCRIPT_TEMP_STORE, DPSCRIPT_RETURN_VAR, DPSCRIPT_TEMP_STORE, temp,
                    ));

                    Ok(b.build())
                }
            }

            Expr::Operation(op) => {
                let temp = "__operation_output";
                let mut out = LineBuilder::from_str(op.compile(state, temp)?);

                out.push(format!(
                    "data modify storage {} {} set from storage {} {}",
                    DPSCRIPT_TEMP_STORE,
                    id.as_ref(),
                    DPSCRIPT_TEMP_STORE,
                    temp,
                ));

                Ok(out.build())
            }

            Expr::Nbt(nbt) => nbt.compile(state),
            Expr::Component(comp) => comp.compile(state),
            Expr::Selector(sel) => sel.compile(state),
            Expr::Var(var) => var.compile(state),

            Expr::Func(func) => {
                func.compile(state)?;

                Ok(String::new())
            }

            Expr::None => unreachable!(),
        }
    }
}
