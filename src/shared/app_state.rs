use std::sync::{Arc, Mutex};
use crate::core::simulation::sim_engine::SimulationEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<SimulationEngine>>,
}
