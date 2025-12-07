//! Safety classification of environment zones.
//!
//! Allows instant death, crush, pressure melt, vacuum exposure events.

use crate::core::env::bodies::EARTH;
use crate::core::uvoxid::UvoxId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentRisk {
    Safe,
    AtmosphereThin,
    VacuumExposure,
    DeepMantle,
    CoreDeath,
}

pub fn check_environment(id: &UvoxId) -> EnvironmentRisk {
    let r_m = id.r_um.meters();

    if r_m < EARTH.radius_m * 0.2 {
        return EnvironmentRisk::CoreDeath;
    }
    if r_m < EARTH.radius_m * 0.9 {
        return EnvironmentRisk::DeepMantle;
    }
    if r_m > EARTH.radius_m + 200_000.0 {
        return EnvironmentRisk::VacuumExposure;
    }
    if r_m > EARTH.radius_m + 50_000.0 {
        return EnvironmentRisk::AtmosphereThin;
    }

    EnvironmentRisk::Safe
}
