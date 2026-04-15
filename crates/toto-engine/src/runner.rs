use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::action::ActionFlow;
use crate::backend::Backend;
use crate::error::EngineError;
use crate::exec::ExecCtx;
use crate::repeat::Repeat;
use crate::script::Script;

/// Factory that constructs a backend *inside* the runner thread.
///
/// `enigo::Enigo` is not `Send` on all platforms, so the engine never ships a
/// constructed backend across threads. The factory is `Send` and is invoked
/// once the new thread starts.
pub type BackendFactory =
    Box<dyn FnOnce() -> Result<Box<dyn Backend>, EngineError> + Send + 'static>;

pub struct ScriptRunner {
    stop_tx: Sender<()>,
    handle: Option<JoinHandle<Result<(), EngineError>>>,
}

impl ScriptRunner {
    pub fn spawn(script: Script, make_backend: BackendFactory) -> Self {
        let (stop_tx, stop_rx) = mpsc::channel();
        let handle = thread::spawn(move || -> Result<(), EngineError> {
            let mut backend = make_backend()?;
            run_script(&script, backend.as_mut(), &stop_rx)
        });
        Self {
            stop_tx,
            handle: Some(handle),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.handle
            .as_ref()
            .map(|h| h.is_finished())
            .unwrap_or(true)
    }

    /// Signal stop and join the thread.
    pub fn stop(mut self) -> Result<(), EngineError> {
        let _ = self.stop_tx.send(());
        if let Some(h) = self.handle.take() {
            h.join().map_err(|_| EngineError::ThreadPanic)??;
        }
        Ok(())
    }
}

fn run_script(
    script: &Script,
    backend: &mut dyn Backend,
    stop_rx: &Receiver<()>,
) -> Result<(), EngineError> {
    let mut ctx = ExecCtx::new(backend, stop_rx);
    let total: Option<u32> = match script.repeat {
        Repeat::Once => Some(1),
        Repeat::Times(n) => Some(n),
        Repeat::Infinite => None,
    };

    let mut iter: u32 = 0;
    loop {
        if let Some(limit) = total {
            if iter >= limit {
                return Ok(());
            }
        }
        for action in &script.actions {
            if ctx.is_stopped() {
                return Ok(());
            }
            match action.execute(&mut ctx)? {
                ActionFlow::Continue => {}
                ActionFlow::Stop => return Ok(()),
            }
        }
        iter = iter.saturating_add(1);
    }
}
