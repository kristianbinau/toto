use enigo::{
    Button as EnButton, Coordinate as EnCoord, Direction as EnDirection, Enigo, Key as EnKey,
    Keyboard, Mouse, Settings,
};

use super::Backend;
use crate::error::EngineError;
use crate::types::{Coord, Direction, Key, MouseButton};

/// Production backend that forwards to [`enigo`].
///
/// `enigo::Enigo` is not `Send` on all platforms, so [`EnigoBackend`] must be
/// constructed *inside* the thread that will use it — see
/// [`crate::runner::BackendFactory`].
pub struct EnigoBackend {
    inner: Enigo,
}

impl EnigoBackend {
    pub fn new() -> Result<Self, EngineError> {
        let inner =
            Enigo::new(&Settings::default()).map_err(|e| EngineError::Backend(e.to_string()))?;
        Ok(Self { inner })
    }
}

impl Backend for EnigoBackend {
    fn click(&mut self, button: MouseButton, direction: Direction) -> Result<(), EngineError> {
        self.inner
            .button(to_btn(button), to_dir(direction))
            .map_err(|e| EngineError::Backend(e.to_string()))
    }

    fn key(&mut self, key: Key, direction: Direction) -> Result<(), EngineError> {
        self.inner
            .key(to_key(key), to_dir(direction))
            .map_err(|e| EngineError::Backend(e.to_string()))
    }

    fn move_mouse(&mut self, x: i32, y: i32, coord: Coord) -> Result<(), EngineError> {
        let c = match coord {
            Coord::Absolute => EnCoord::Abs,
            Coord::Relative => EnCoord::Rel,
        };
        self.inner
            .move_mouse(x, y, c)
            .map_err(|e| EngineError::Backend(e.to_string()))
    }
}

fn to_dir(d: Direction) -> EnDirection {
    match d {
        Direction::Press => EnDirection::Press,
        Direction::Release => EnDirection::Release,
        Direction::Click => EnDirection::Click,
    }
}

fn to_btn(b: MouseButton) -> EnButton {
    match b {
        MouseButton::Left => EnButton::Left,
        MouseButton::Right => EnButton::Right,
        MouseButton::Middle => EnButton::Middle,
    }
}

fn to_key(k: Key) -> EnKey {
    match k {
        Key::Unicode(c) => EnKey::Unicode(c),
        Key::Return => EnKey::Return,
        Key::Tab => EnKey::Tab,
        Key::Space => EnKey::Space,
        Key::Escape => EnKey::Escape,
        Key::Backspace => EnKey::Backspace,
    }
}
