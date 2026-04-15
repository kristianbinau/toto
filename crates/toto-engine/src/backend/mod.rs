use crate::error::EngineError;
use crate::types::{Coord, Direction, Key, MouseButton};

pub mod mock;

#[cfg(feature = "enigo-backend")]
pub mod enigo_backend;

pub use mock::{MockBackend, MockEvent};

#[cfg(feature = "enigo-backend")]
pub use enigo_backend::EnigoBackend;

/// Abstract side-effect surface used by every action.
///
/// Implementations are only used from within the runner's own thread — they
/// do not need to be [`Send`]. The factory closure that constructs the
/// backend (see [`crate::runner::BackendFactory`]) is what crosses the thread
/// boundary and is required to be `Send`.
pub trait Backend {
    fn click(&mut self, button: MouseButton, direction: Direction) -> Result<(), EngineError>;
    fn key(&mut self, key: Key, direction: Direction) -> Result<(), EngineError>;
    fn move_mouse(&mut self, x: i32, y: i32, coord: Coord) -> Result<(), EngineError>;
}
