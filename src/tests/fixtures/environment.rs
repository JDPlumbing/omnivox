use crate::core::environment::state::EnvironmentState;
use crate::core::environment::components::{
    atmosphere::AtmosphereDescriptor,
    composition::{AtmosphericComposition, GasFraction},
};
use crate::core::worlds::id::WorldId;
use crate::core::physics::units::{
    pressure::Pascals,
    length::Meters,
    albedo::Albedo,
};

/// Canonical Earth-like environment descriptor for tests.
///
/// Explicit by design:
/// no helpers, no defaults, no magic.
pub fn earth_like_environment(world_id: WorldId) -> EnvironmentState {
    let mut env = EnvironmentState::default();

    // --- Atmosphere descriptor ---
    env.atmospheres.insert(
        world_id,
        AtmosphereDescriptor {
            albedo: Albedo(0.30),
            surface_pressure: Pascals(101_325.0),
            scale_height: Meters(8_500.0), // ~8.5 km
        },
    );

    // --- Atmospheric composition ---
    env.compositions.insert(
        world_id,
        AtmosphericComposition {
            gases: vec![
                GasFraction {
                    name: "N2".to_string(),
                    molar_fraction: 0.78084,
                },
                GasFraction {
                    name: "O2".to_string(),
                    molar_fraction: 0.20946,
                },
                GasFraction {
                    name: "Ar".to_string(),
                    molar_fraction: 0.00934,
                },
                GasFraction {
                    name: "CO2".to_string(),
                    molar_fraction: 0.00036,
                },
            ],
        },
    );

    env
}
