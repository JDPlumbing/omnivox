// core/entity/components/material/emissivity.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Emissivity(pub f64); // 0.0 ..= 1.0
