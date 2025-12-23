use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalData {
    pub temperature_c: f64,                  // instantaneous temp
    pub heat_capacity_j_per_kg_k: f64,
    pub absorptivity: f64,
    pub mass_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalExposure {
    pub total_energy_j: f64,                 // accumulated absorbed energy
    pub average_temperature_c: f64,          // rolling mean
    pub cycles: u64,                         // number of integration steps
}
