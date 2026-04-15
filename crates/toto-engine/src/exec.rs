use std::sync::mpsc::{Receiver, RecvTimeoutError, TryRecvError};
use std::time::Duration;

use crate::backend::Backend;

/// Per-action execution context.
///
/// Carries a mutable backend reference and a stop-signal receiver so that
/// long-running actions (notably [`crate::action::Delay`]) can be interrupted
/// promptly.
pub struct ExecCtx<'a> {
    backend: &'a mut dyn Backend,
    stop_rx: &'a Receiver<()>,
}

impl<'a> ExecCtx<'a> {
    pub fn new(backend: &'a mut dyn Backend, stop_rx: &'a Receiver<()>) -> Self {
        Self { backend, stop_rx }
    }

    pub fn backend(&mut self) -> &mut dyn Backend {
        self.backend
    }

    pub fn is_stopped(&self) -> bool {
        matches!(
            self.stop_rx.try_recv(),
            Ok(()) | Err(TryRecvError::Disconnected)
        )
    }

    /// Sleep for `d`, returning `true` if stop was signaled during the sleep.
    pub fn sleep(&self, d: Duration) -> bool {
        match self.stop_rx.recv_timeout(d) {
            Ok(()) => true,
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        }
    }
}
