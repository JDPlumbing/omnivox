use crate::core::uvoxid::UvoxId;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position(pub UvoxId);

