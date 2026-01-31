use crate::core::physics::units::{
    temperature::Kelvin,
    pressure::Pascals,

    irradiance::WattsPerSquareMeter,
    acceleration::MetersPerSecondSquared,
};

#[derive(Debug, Clone, Copy)]
pub struct EnvironmentConditions {
    pub temperature: Kelvin,
    pub pressure: Pascals,

    /// Raw insolation at the surface (W/m²)
    pub insolation: WattsPerSquareMeter,
    pub gravity: MetersPerSecondSquared,
}

impl Default for EnvironmentConditions {
    fn default() -> Self {
        Self {
            temperature: Kelvin(288.15),          // ~15°C
            pressure: Pascals(101_325.0),         // sea-level Earth
            insolation: WattsPerSquareMeter(0.0), // night / placeholder
            gravity: MetersPerSecondSquared(9.81),
        }
    }
}
