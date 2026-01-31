// core/entity/components/mass.rs
use crate::core::physics::units::mass::Kilograms;

#[derive(Debug, Clone, Copy)]
pub struct Mass(pub Kilograms);
