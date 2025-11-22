use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UVDegradationData {
    pub cumulative_uv_j_m2: f64,
    pub severity: f64,
    pub rate_m_per_year: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UVExposure {
    pub total_energy_j: f64,                 // accumulated absorbed energy
    pub average_temperature_c: f64,          // rolling mean
    pub cycles: u64,                         // number of integration steps
}
