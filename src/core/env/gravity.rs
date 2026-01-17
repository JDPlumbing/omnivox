use crate::core::env::fields::{Field, FieldSample};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::SimDuration;
use crate::core::world::world_env_descriptor::{WorldSpace, GravityModel, GravityKind};


pub struct GravityField {
    pub gravity_radial: f64, // m/s² toward -r
}

impl GravityField {
    pub fn from_model(_space: &WorldSpace, gravity: &GravityModel) -> Self {
        let g = match gravity.kind {
            GravityKind::Radial => gravity.strength,
            GravityKind::Uniform { .. } => gravity.strength,
            GravityKind::None => 0.0,
        };

        Self {
            gravity_radial: g,
        }
    }
}
impl Field for GravityField {
    fn sample(&self, _id: &UvoxId, _time: SimDuration) -> FieldSample {
        FieldSample {
            gravity_radial: self.gravity_radial,
            ..Default::default()
        }
    }
}