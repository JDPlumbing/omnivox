use serde::{Serialize, Deserialize};
use crate::core::tdt::sim_time::SimTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UVDegradationData {
    /// Total accumulated UV dose (J/m² equivalent)
    pub total_uv_dose: f64,

    /// Number of thermal cycles (daily expansion/contraction)
    pub thermal_cycles: f64,

    /// Last temperature sign for cycle detection
    pub last_temp_sign: i8,

    /// Last update time
    pub last_update: SimTime,
}

impl UVDegradationData {
    pub fn new(t: SimTime) -> Self {
        Self {
            total_uv_dose: 0.0,
            thermal_cycles: 0.0,
            last_temp_sign: 0,
            last_update: t,
        }
    }
}
