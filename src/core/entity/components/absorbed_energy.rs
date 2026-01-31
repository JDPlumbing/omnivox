// entity/components/aborbed_energy.rs
use crate::core::physics::units::energy::Joules;

#[derive(Debug, Clone, Copy, Default)]
pub struct AbsorbedEnergy {
    /// Cumulative radiant energy received (J)
    pub joules: Joules,
}
