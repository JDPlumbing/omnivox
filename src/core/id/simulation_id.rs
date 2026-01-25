use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimulationId(pub Uuid);

impl SimulationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SimulationId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for SimulationId {
    fn from(u: Uuid) -> Self {
        Self(u)
    }
}

impl std::fmt::Display for SimulationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SimulationId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}
