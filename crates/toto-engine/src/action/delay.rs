use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::action::{ActionFlow, ActionResult};
use crate::exec::ExecCtx;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Delay {
    pub ms: u64,
}

impl Delay {
    /// OS schedulers are unreliable below ~10ms; clamp.
    pub const MIN_MS: u64 = 10;

    pub fn execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult {
        let ms = self.ms.max(Self::MIN_MS);
        if ctx.sleep(Duration::from_millis(ms)) {
            Ok(ActionFlow::Stop)
        } else {
            Ok(ActionFlow::Continue)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::MockBackend;
    use std::sync::mpsc;
    use std::time::Instant;

    #[test]
    fn delay_clamps_to_minimum() {
        let mut backend = MockBackend::new();
        let (_tx, rx) = mpsc::channel();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        let start = Instant::now();
        let flow = Delay { ms: 0 }.execute(&mut ctx).unwrap();
        let elapsed = start.elapsed();

        assert!(matches!(flow, ActionFlow::Continue));
        assert!(elapsed >= Duration::from_millis(Delay::MIN_MS));
    }

    #[test]
    fn delay_returns_stop_when_signaled() {
        let mut backend = MockBackend::new();
        let (tx, rx) = mpsc::channel();
        tx.send(()).unwrap();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        let flow = Delay { ms: 10_000 }.execute(&mut ctx).unwrap();
        assert!(matches!(flow, ActionFlow::Stop));
    }
}
