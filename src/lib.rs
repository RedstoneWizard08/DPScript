use error::CompilationError;

pub mod call;
pub mod comp;
pub mod compiler;
pub mod config;
pub mod error;
pub mod expr;
pub mod func;
pub mod lines;
pub mod macros;
pub mod nbt;
pub mod op;
pub mod parser;
pub mod selector;
pub mod state;
pub mod var;

pub const DPSCRIPT_VAR_STORE: &str = "dpscript:builtin/stores/vars";
pub const DPSCRIPT_TEMP_STORE: &str = "dpscript:builtin/stores/temp";
pub const DPSCRIPT_DUMMY_OBJECTIVE: &str = "__dpscript_temp";
pub const DPSCRIPT_RETURN_VAR: &str = "__return_val";
pub const DPSCRIPT_FLOAT_PRECISION_MUL: i32 = 100000000;

pub type Result<T, E = CompilationError> = core::result::Result<T, E>;
