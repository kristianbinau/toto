use serde::{Deserialize, Serialize};

use crate::action::{ActionFlow, ActionResult};
use crate::exec::ExecCtx;
use crate::types::{Direction, Key};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KeyPress {
    pub key: Key,
    pub direction: Direction,
}

impl KeyPress {
    pub fn execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult {
        ctx.backend().key(self.key, self.direction)?;
        Ok(ActionFlow::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{MockBackend, MockEvent};
    use std::sync::mpsc;

    #[test]
    fn unicode_keypress_records_event() {
        let mut backend = MockBackend::new();
        let handle = backend.clone();
        let (_tx, rx) = mpsc::channel();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        KeyPress {
            key: Key::Unicode('a'),
            direction: Direction::Click,
        }
        .execute(&mut ctx)
        .unwrap();

        assert_eq!(
            handle.events(),
            vec![MockEvent::Key(Key::Unicode('a'), Direction::Click)]
        );
    }
}
