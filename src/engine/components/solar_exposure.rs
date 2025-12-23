use serde::{Serialize, Deserialize};

/// Instantaneous solar exposure (per tick)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarExposure {
    /// Watts per square meter reaching the surface (0–1361)
    pub irradiance_w_m2: f64,

    /// UV index-like scalar (0–1 scaled)
    pub uv_intensity: f64,

    /// Temperature rise contribution for this tick
    pub temp_delta_c: f64,
}

impl SolarExposure {
    pub fn new() -> Self {
        Self {
            irradiance_w_m2: 0.0,
            uv_intensity: 0.0,
            temp_delta_c: 0.0,
        }
    }
}
