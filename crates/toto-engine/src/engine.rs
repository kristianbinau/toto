use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::backend::Backend;
use crate::error::EngineError;
use crate::runner::{BackendFactory, ScriptRunner};
use crate::script::{Script, ScriptId};

/// Shared factory used to construct one backend per running script.
pub type MakeBackend =
    Arc<dyn Fn() -> Result<Box<dyn Backend>, EngineError> + Send + Sync + 'static>;

/// Top-level facade that owns zero or more concurrently running scripts.
///
/// Each running script lives in its own thread with its own backend instance,
/// so scripts are fully independent — starting, stopping, or toggling one
/// script never affects the others.
pub struct Engine {
    running: Mutex<HashMap<ScriptId, ScriptRunner>>,
    factory: MakeBackend,
}

impl Engine {
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> Result<Box<dyn Backend>, EngineError> + Send + Sync + 'static,
    {
        Self {
            running: Mutex::new(HashMap::new()),
            factory: Arc::new(factory),
        }
    }

    pub fn start(&self, script: Script) -> Result<(), EngineError> {
        let mut map = self.running.lock().unwrap();
        if let Some(existing) = map.get(&script.id) {
            if !existing.is_finished() {
                return Err(EngineError::AlreadyRunning(script.id.0.clone()));
            }
            if let Some(old) = map.remove(&script.id) {
                let _ = old.stop();
            }
        }
        let id = script.id.clone();
        let factory = Arc::clone(&self.factory);
        let once: BackendFactory = Box::new(move || (factory)());
        let runner = ScriptRunner::spawn(script, once);
        map.insert(id, runner);
        Ok(())
    }

    pub fn stop(&self, id: &ScriptId) -> Result<(), EngineError> {
        let runner = {
            let mut map = self.running.lock().unwrap();
            map.remove(id)
                .ok_or_else(|| EngineError::NotRunning(id.0.clone()))?
        };
        runner.stop()
    }

    /// Start if not running, stop if running. Returns `true` if the script is
    /// now active, `false` if it was just stopped.
    pub fn toggle(&self, script: Script) -> Result<bool, EngineError> {
        let id = script.id.clone();
        let active = {
            let map = self.running.lock().unwrap();
            map.get(&id).map(|r| !r.is_finished()).unwrap_or(false)
        };
        if active {
            self.stop(&id)?;
            Ok(false)
        } else {
            self.start(script)?;
            Ok(true)
        }
    }

    pub fn is_running(&self, id: &ScriptId) -> bool {
        let map = self.running.lock().unwrap();
        map.get(id).map(|r| !r.is_finished()).unwrap_or(false)
    }

    pub fn running_ids(&self) -> Vec<ScriptId> {
        self.running
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, r)| !r.is_finished())
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn stop_all(&self) {
        let runners: Vec<ScriptRunner> = {
            let mut map = self.running.lock().unwrap();
            map.drain().map(|(_, r)| r).collect()
        };
        for r in runners {
            let _ = r.stop();
        }
    }
}
