use crate::core::physics::units::{pressure::Pascals, albedo::Albedo};

pub struct AtmosphereDescriptor {
    pub surface_pressure: Pascals,
    pub albedo: Albedo,
}
