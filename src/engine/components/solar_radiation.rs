use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarRadiation {
    pub luminosity_w: f64,
    pub uv_fraction: f64,   // 0–1 fraction of radiant energy in UV
}

impl SolarRadiation {
    pub fn sun_default() -> Self {
        Self {
            luminosity_w: 3.828e26,
            uv_fraction: 0.07,
        }
    }
}
