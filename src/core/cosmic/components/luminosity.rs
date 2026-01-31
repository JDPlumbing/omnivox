// core/cosmic/components/luminosity.rs
use crate::core::physics::units::power::Watts;

#[derive(Debug, Clone, Copy)]
pub struct Luminosity {
    pub watts: Watts,
}
