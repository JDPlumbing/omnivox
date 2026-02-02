use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KilogramsPerCubicMeter(pub f64);
