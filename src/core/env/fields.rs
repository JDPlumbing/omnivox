use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::env::medium::Medium;


pub trait Field: Send + Sync {
    fn sample(&self, _id: &UvoxId, _time: SimDuration) -> FieldSample {
        FieldSample::default()
    }

    fn derive(
        &self,
        _id: &UvoxId,
        _time: SimDuration,
        _env: &FieldSample,
    ) -> FieldSample {
        FieldSample::default()
    }
}


#[derive(Debug, Clone)]
pub struct FieldSample {
    // Always-defined baseline
    pub medium: Medium,        // Solid / Liquid / Gas / Vacuum
    pub density: f64,          // kg/m³
    pub pressure: f64,         // Pa
    pub temperature: f64,      // K
    pub gravity_radial: f64,   // m/s² (toward -r)

    // Optional / additive influences
    pub wind_radial: f64,      // m/s
    pub resistance: f64,       // drag-ish scalar
    pub land_height_m: f64,
}

impl Default for FieldSample {
    fn default() -> Self {
        Self {
            medium: Medium::Vacuum,
            density: 0.0,
            pressure: 0.0,
            temperature: 0.0,
            gravity_radial: 0.0,
            wind_radial: 0.0,
            resistance: 0.0,
            land_height_m: 0.0,
        }
    }
}

impl FieldSample {
    pub fn merge(&mut self, other: FieldSample) {
        if other.medium != Medium::Vacuum {
            self.medium = other.medium;
        }

        self.density += other.density;
        self.pressure += other.pressure;
        self.temperature += other.temperature;
        self.gravity_radial += other.gravity_radial;
        self.wind_radial += other.wind_radial;
        self.resistance += other.resistance;
    }
}
