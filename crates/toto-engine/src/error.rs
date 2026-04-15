use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("script `{0}` is already running")]
    AlreadyRunning(String),

    #[error("script `{0}` is not running")]
    NotRunning(String),

    #[error("backend error: {0}")]
    Backend(String),

    #[error("runner thread panicked")]
    ThreadPanic,
}
