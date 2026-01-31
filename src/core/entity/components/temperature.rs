// entity/components/temperature.rs
use crate::core::physics::units::temperature::Kelvin;

#[derive(Debug, Clone, Copy)]
pub struct Temperature(pub Kelvin);
