use error::Error;

pub mod cli;
pub mod compiler;
pub mod config;
pub mod error;
pub mod expr;
pub mod format;
pub mod lines;
pub mod macros;
pub mod parser;
pub mod state;

pub const DPSCRIPT_VAR_STORE: &str = "dpscript:builtin/stores/vars";
pub const DPSCRIPT_TEMP_STORE: &str = "dpscript:builtin/stores/temp";
pub const DPSCRIPT_DUMMY_OBJECTIVE: &str = "__dpscript_temp";
pub const DPSCRIPT_RETURN_VAR: &str = "__return_val";
pub const DPSCRIPT_FLOAT_PRECISION_MUL: i32 = 100000000;

pub type Result<T, E = Error> = core::result::Result<T, E>;
