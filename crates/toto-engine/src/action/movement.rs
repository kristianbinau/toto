use serde::{Deserialize, Serialize};

use crate::action::{ActionFlow, ActionResult};
use crate::exec::ExecCtx;
use crate::types::Coord;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MouseMove {
    pub x: i32,
    pub y: i32,
    pub coord: Coord,
}

impl MouseMove {
    pub fn execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult {
        ctx.backend().move_mouse(self.x, self.y, self.coord)?;
        Ok(ActionFlow::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{MockBackend, MockEvent};
    use std::sync::mpsc;

    #[test]
    fn move_records_event() {
        let mut backend = MockBackend::new();
        let handle = backend.clone();
        let (_tx, rx) = mpsc::channel();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        MouseMove {
            x: 100,
            y: 200,
            coord: Coord::Absolute,
        }
        .execute(&mut ctx)
        .unwrap();

        assert_eq!(
            handle.events(),
            vec![MockEvent::Move(100, 200, Coord::Absolute)]
        );
    }
}
