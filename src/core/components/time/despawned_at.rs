use crate::core::SimTime;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DespawnedAt {
    pub time: SimTime,
}
