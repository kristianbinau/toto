use std::sync::mpsc::{Receiver, RecvTimeoutError, TryRecvError};
use std::time::{Duration, Instant};

use crate::backend::Backend;

// Delays at or below this threshold use a pure spin-wait for sub-ms precision.
const SPIN_THRESHOLD: Duration = Duration::from_millis(2);
// Delays up to this threshold use a hybrid: coarse recv_timeout + spin tail.
const HYBRID_THRESHOLD: Duration = Duration::from_millis(20);
// How much to spin at the end of a hybrid sleep to absorb scheduler overshoot.
const SPIN_TAIL: Duration = Duration::from_millis(1);

/// `recv_timeout` wrapped with `timeBeginPeriod(1)` on Windows so that the OS
/// multimedia timer fires at 1 ms resolution instead of the default ~15.6 ms.
/// Without this, a 20 ms sleep can overshoot to ~31 ms on a stock Windows box.
/// On non-Windows platforms this is just a direct `recv_timeout` call.
fn recv_timeout_hi_res(rx: &Receiver<()>, d: Duration) -> Result<(), RecvTimeoutError> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Media::{timeBeginPeriod, timeEndPeriod};
        // SAFETY: timeBeginPeriod/timeEndPeriod are thread-safe Win32 calls with
        // no preconditions beyond pairing each Begin with an End.
        unsafe { timeBeginPeriod(1) };
        let result = rx.recv_timeout(d);
        unsafe { timeEndPeriod(1) };
        result
    }
    #[cfg(not(windows))]
    {
        rx.recv_timeout(d)
    }
}

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
    ///
    /// Three tiers:
    /// - `d <= SPIN_THRESHOLD` (≤2 ms): pure spin-wait — precise but CPU-intensive.
    /// - `d <= HYBRID_THRESHOLD` (≤20 ms): coarse `recv_timeout` for most of the
    ///   duration, then spin the last `SPIN_TAIL` (1 ms) to absorb any remaining
    ///   scheduler overshoot.
    /// - Otherwise: pure `recv_timeout` — CPU-friendly for long delays.
    ///
    /// On Windows, `recv_timeout` calls go through [`recv_timeout_hi_res`] which
    /// temporarily sets the multimedia timer to 1 ms resolution, preventing the
    /// ~13 ms overshoot seen at default (15.6 ms) timer granularity.
    pub fn sleep(&self, d: Duration) -> bool {
        if d <= SPIN_THRESHOLD {
            let deadline = Instant::now() + d;
            loop {
                match self.stop_rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => return true,
                    Err(TryRecvError::Empty) => {}
                }
                if Instant::now() >= deadline {
                    return false;
                }
                std::hint::spin_loop();
            }
        } else if d <= HYBRID_THRESHOLD {
            let coarse = d.saturating_sub(SPIN_TAIL);
            match recv_timeout_hi_res(self.stop_rx, coarse) {
                Ok(()) | Err(RecvTimeoutError::Disconnected) => return true,
                Err(RecvTimeoutError::Timeout) => {}
            }
            let deadline = Instant::now() + SPIN_TAIL;
            loop {
                match self.stop_rx.try_recv() {
                    Ok(()) | Err(TryRecvError::Disconnected) => return true,
                    Err(TryRecvError::Empty) => {}
                }
                if Instant::now() >= deadline {
                    return false;
                }
                std::hint::spin_loop();
            }
        } else {
            match recv_timeout_hi_res(self.stop_rx, d) {
                Ok(()) | Err(RecvTimeoutError::Disconnected) => true,
                Err(RecvTimeoutError::Timeout) => false,
            }
        }
    }
}
