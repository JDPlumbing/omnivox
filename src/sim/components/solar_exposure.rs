use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarExposureData {
    pub energy_j_m2: f64,
    pub uv_j_m2: f64,
}
