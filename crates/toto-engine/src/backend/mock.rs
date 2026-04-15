use std::sync::{Arc, Mutex};

use super::Backend;
use crate::error::EngineError;
use crate::types::{Coord, Direction, Key, MouseButton};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MockEvent {
    Click(MouseButton, Direction),
    Key(Key, Direction),
    Move(i32, i32, Coord),
}

/// Test backend that records every call into a shared, cloneable buffer.
///
/// Clones share the same underlying buffer, so a handle can be kept outside
/// the runner thread to assert on events while the runner is still alive.
#[derive(Clone, Default)]
pub struct MockBackend {
    events: Arc<Mutex<Vec<MockEvent>>>,
}

impl MockBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> Vec<MockEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn len(&self) -> usize {
        self.events.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.lock().unwrap().is_empty()
    }

    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
    }
}

impl Backend for MockBackend {
    fn click(&mut self, button: MouseButton, direction: Direction) -> Result<(), EngineError> {
        self.events
            .lock()
            .unwrap()
            .push(MockEvent::Click(button, direction));
        Ok(())
    }

    fn key(&mut self, key: Key, direction: Direction) -> Result<(), EngineError> {
        self.events
            .lock()
            .unwrap()
            .push(MockEvent::Key(key, direction));
        Ok(())
    }

    fn move_mouse(&mut self, x: i32, y: i32, coord: Coord) -> Result<(), EngineError> {
        self.events
            .lock()
            .unwrap()
            .push(MockEvent::Move(x, y, coord));
        Ok(())
    }
}
