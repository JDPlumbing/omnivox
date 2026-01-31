use crate::core::spatial::uvox_id::UvoxId;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position(pub UvoxId);

