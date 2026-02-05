use crate::core::spatial::uvox_id::UvoxId;

use serde::{Serialize, Deserialize};

// Persistent spatial anchor (identity), not a physical position
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position(pub UvoxId);

