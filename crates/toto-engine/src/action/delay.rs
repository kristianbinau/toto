use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::action::{ActionFlow, ActionResult};
use crate::exec::ExecCtx;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delay {
    pub ms: f64,
}

impl Delay {
    pub fn execute(&self, ctx: &mut ExecCtx<'_>) -> ActionResult {
        let d = Duration::from_secs_f64(self.ms.max(0.0) / 1000.0);
        if ctx.sleep(d) {
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
    fn delay_zero_completes_instantly() {
        let mut backend = MockBackend::new();
        let (_tx, rx) = mpsc::channel();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        let start = Instant::now();
        let flow = Delay { ms: 0.0 }.execute(&mut ctx).unwrap();
        let elapsed = start.elapsed();

        assert!(matches!(flow, ActionFlow::Continue));
        assert!(elapsed < Duration::from_millis(1));
    }

    #[test]
    fn delay_sub_ms_sleeps_approximately() {
        let mut backend = MockBackend::new();
        let (_tx, rx) = mpsc::channel();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        let start = Instant::now();
        let flow = Delay { ms: 0.5 }.execute(&mut ctx).unwrap();
        let elapsed = start.elapsed();

        assert!(matches!(flow, ActionFlow::Continue));
        // Should complete within a reasonable window around 0.5ms
        assert!(elapsed < Duration::from_millis(5));
    }

    #[test]
    fn delay_returns_stop_when_signaled() {
        let mut backend = MockBackend::new();
        let (tx, rx) = mpsc::channel();
        tx.send(()).unwrap();
        let mut ctx = ExecCtx::new(&mut backend, &rx);

        let flow = Delay { ms: 10_000.0 }.execute(&mut ctx).unwrap();
        assert!(matches!(flow, ActionFlow::Stop));
    }
}
