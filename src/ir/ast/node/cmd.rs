use serde::{Deserialize, Serialize};

use super::IRNode;

/// A command.
/// This exists so I can implement methods for it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IRCommand {
    /// The command string.
    pub cmd: Vec<IRNode>,
}
