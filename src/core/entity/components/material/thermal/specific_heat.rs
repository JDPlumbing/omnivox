// entity/components/material/specific_heat.rs
use crate::core::physics::units::specific_heat::JoulesPerKilogramKelvin;

#[derive(Debug, Clone, Copy)]
pub struct SpecificHeat(pub JoulesPerKilogramKelvin);
