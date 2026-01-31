use crate::core::physics::units::{length::Meters, pressure::Pascals, albedo::Albedo};

pub struct AtmosphereDescriptor {
    pub scale_height: Meters,
    pub surface_pressure: Pascals,
    pub albedo: Albedo,
}
