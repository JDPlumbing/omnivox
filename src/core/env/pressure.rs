use crate::core::env::fields::{Field, FieldSample};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::SimDuration;

/// Simple hydrostatic pressure approximation
pub struct PressureField {
    /// Reference pressure at sea level (Pa)
    pub sea_level_pressure: f64,
}

impl Field for PressureField {
    fn sample(&self, id: &UvoxId, _time: SimDuration) -> FieldSample {
        // Pressure depends on *other* fields, so this will be filled later
        // by WorldEnvironment after merge.
        FieldSample {
            pressure: self.sea_level_pressure,
            ..Default::default()
        }
    }
}
