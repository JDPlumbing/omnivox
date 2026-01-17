use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::env::fields::{Field, FieldSample};

/// Fields are merged in order.
/// Later fields override earlier ones.
/// WorldEnvironment.fields should be ordered from global → local.

use std::sync::Arc;

#[derive(Clone)]
pub struct WorldEnvironment {
    pub fields: Vec<Arc<dyn Field>>,
}

impl WorldEnvironment {
    pub fn sample(&self, id: &UvoxId, time: SimDuration) -> FieldSample {
        let mut sample = self.fields.iter().fold(
            FieldSample::default(),
            |acc, field| acc.merge(field.sample(id, time)),
        );

        // Resolve pressure if possible
        if sample.pressure != 0.0 {
            if let (density, gravity_radial) =
                (sample.density, sample.gravity_radial)
            {
                // Project gravity onto radial direction
                let g_mag = gravity_radial;
                // Simple hydrostatic approximation:
                // P ≈ ρ * g * h
                // Here h is implicitly encoded in density falloff,
                // so we just ensure pressure trends correctly.
                sample.pressure = sample.pressure * density * g_mag;
            }
        }

        sample
    }
}
