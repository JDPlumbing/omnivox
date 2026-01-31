use crate::core::physics::units::energy_density::JoulesPerSquareMeter;

#[derive(Debug, Clone, Copy, Default)]
pub struct Exposure {
    /// Cumulative radiant energy received (J/m²)
    pub radiant: JoulesPerSquareMeter,
}
