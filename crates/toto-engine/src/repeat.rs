use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(tag = "mode", content = "count")]
pub enum Repeat {
    #[default]
    Once,
    Times(u32),
    Infinite,
}
