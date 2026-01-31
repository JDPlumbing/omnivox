// entity/components/internal_energy.rs
use crate::core::physics::units::energy::Joules;

#[derive(Debug, Clone, Copy, Default)]
pub struct InternalEnergy {
    pub joules: Joules,
}
