use serde::{Deserialize, Serialize};

use crate::error::EngineError;
use crate::exec::ExecCtx;

pub mod click;
pub mod delay;
pub mod keypress;
pub mod movement;

pub use click::MouseClick;
pub use delay::Delay;
pub use keypress::KeyPress;
pub use movement::MouseMove;

/// Result of executing a single action.
#[derive(Debug)]
pub enum ActionFlow {
    /// Continue with the next action.
    Continue,
    /// Stop the script cleanly (e.g. stop was signaled mid-delay).
    Stop,
}

pub type ActionResult = Result<ActionFlow, EngineError>;

/// A single step in a [`crate::script::Script`].
///
/// Adding a new action type means:
/// 1. Create a new module under `action/` with a struct + `execute`.
/// 2. Add a variant here and a match arm in [`Action::execute`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    Click(MouseClick),
    Key(KeyPress),
    Move(MouseMove),
    Delay(Delay),
}

impl Action {
    pub fn execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult {
        match self {
            Action::Click(a) => a.execute(ctx),
            Action::Key(a) => a.execute(ctx),
            Action::Move(a) => a.execute(ctx),
            Action::Delay(a) => a.execute(ctx),
        }
    }
}
