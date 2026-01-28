// src/core/world/catalog.rs

use crate::core::tdt::sim_time::SimTime;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWorld {
    pub name: String,
    pub description: Option<String>,
    pub environment: Option<serde_json::Value>,
    pub world_epoch: Option<SimTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub environment: Option<serde_json::Value>,
}
