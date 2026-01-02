use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;
use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId {
    pub index: u32,
    pub generation: u32,
}

impl EntityId {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}

/// Human-readable + DB-storable form: "index:generation"
impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.index, self.generation)
    }
}

/// Parse EntityId from "index:generation"
impl FromStr for EntityId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("invalid entity id '{}'", s));
        }

        Ok(EntityId {
            index: parts[0].parse()?,
            generation: parts[1].parse()?,
        })
    }
}

impl EntityId {
    /// TEMP: used only during early prototyping
    pub fn provisional(index: u32) -> Self {
        Self {
            index,
            generation: 0,
        }
    }
}
