use serde::{Deserialize, Serialize};

use crate::action::{ActionFlow, ActionResult};
use crate::exec::ExecCtx;
use crate::types::{Direction, MouseButton};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MouseClick {
    pub button: MouseButton,
    pub direction: Direction,
}

impl MouseClick {
    pub fn execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult {
        ctx.backend().click(self.button, self.direction)?;
        Ok(ActionFlow::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{MockBackend, MockEvent};
    use std::sync::mpsc;

    #[test]
    fn left_click_records_event() {
        let mut backend = MockBackend::new();
        let handle = backend.clone();
        let (_tx, rx) = mpsc::channel();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        MouseClick {
            button: MouseButton::Left,
            direction: Direction::Click,
        }
        .execute(&mut ctx)
        .unwrap();

        assert_eq!(
            handle.events(),
            vec![MockEvent::Click(MouseButton::Left, Direction::Click)]
        );
    }
}
