use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::env::fields::{Field, FieldSample};


pub struct TemperatureField {
    pub surface_temp_k: f64,
    /// Positive = temperature decreases with altitude (Earth-like)
    /// Negative = temperature increases with altitude (inversion)
    pub lapse_rate_k_per_m: f64, // K per meter
    pub surface_radius_m: f64,
}

impl Field for TemperatureField {
    fn sample(&self, id: &UvoxId, _time: SimDuration) -> FieldSample {
        let r_m = id.r_um.meters();
        let alt = r_m - self.surface_radius_m;

        let temp = self.surface_temp_k - alt * self.lapse_rate_k_per_m;

        FieldSample {
            temperature: temp.max(0.0),
            ..Default::default()
        }
    }
}
