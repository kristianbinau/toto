use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::repeat::Repeat;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScriptId(pub String);

impl ScriptId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ScriptId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ScriptId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: ScriptId,
    pub actions: Vec<Action>,
    #[serde(default)]
    pub repeat: Repeat,
}
