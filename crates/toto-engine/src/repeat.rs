use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "mode", content = "count")]
pub enum Repeat {
    Once,
    Times(u32),
    Infinite,
}

impl Default for Repeat {
    fn default() -> Self {
        Repeat::Once
    }
}
