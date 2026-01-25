use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct WorldId(pub i64);

impl From<i64> for WorldId {
    fn from(value: i64) -> Self {
        WorldId(value)
    }
}

use std::fmt;

impl fmt::Display for WorldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

