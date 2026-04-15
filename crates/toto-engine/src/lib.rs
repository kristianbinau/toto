//! toto-engine: headless automation engine.
//!
//! The crate is UI-agnostic. It exposes an [`Engine`] that runs [`Script`]s
//! against a swappable [`Backend`]. Side effects (clicks, key presses, mouse
//! movement, sleeping) are routed through the backend so the engine can be
//! unit-tested with [`backend::MockBackend`] without touching a real display.

pub mod action;
pub mod backend;
pub mod engine;
pub mod error;
pub mod exec;
pub mod repeat;
pub mod runner;
pub mod script;
pub mod types;

pub use action::Action;
pub use backend::{Backend, MockBackend};
pub use engine::Engine;
pub use error::EngineError;
pub use repeat::Repeat;
pub use script::{Script, ScriptId};
pub use types::{Coord, Direction, Key, MouseButton};

#[cfg(feature = "enigo-backend")]
pub use backend::EnigoBackend;
