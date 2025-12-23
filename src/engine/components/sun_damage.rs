use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunDamage {
    pub total_irradiance_j_m2: f64,
    pub total_uv_j_m2: f64,
    pub total_temp_delta: f64,

    pub thermal_cycles: f64,
    pub last_temp_sign: i8,
}

impl SunDamage {
    pub fn new() -> Self {
        Self {
            total_irradiance_j_m2: 0.0,
            total_uv_j_m2: 0.0,
            total_temp_delta: 0.0,
            thermal_cycles: 0.0,
            last_temp_sign: 0,
        }
    }
}

