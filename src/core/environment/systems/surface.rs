use crate::core::environment::{
    state::EnvironmentState,
    conditions::EnvironmentConditions,
    systems::{
        thermal::equilibrium_temperature,
        atmosphere::mean_molecular_weight,
        scale_height::scale_height,
        barometric::pressure_at_altitude,
    },
};
use crate::core::worlds::systems::insolation::compute_insolation;
use crate::core::worlds::systems::surface::WorldSurfaceSample;
use crate::core::worlds::systems::radiation::SurfaceRadiation;
use crate::core::worlds::id::WorldId;
use crate::core::physics::units::{
 
    acceleration::MetersPerSecondSquared,
};

pub fn environment_at_surface(
    world_id: WorldId,
    surface: &WorldSurfaceSample,
    radiation: &SurfaceRadiation,
    gravity: MetersPerSecondSquared,
    env: &EnvironmentState,
) -> EnvironmentConditions {
    // --- Insolation ---
    let insolation = compute_insolation(
        radiation.direction_local,
        radiation.flux,
        surface.normal_local,
    );

    // --- Atmosphere ---
    let atmosphere = env.atmospheres
        .get(&world_id)
        .expect("world has no atmosphere");

    let composition = env.compositions
        .get(&world_id)
        .expect("world has no atmospheric composition");

    // --- Thermodynamics ---
    let temperature = equilibrium_temperature(
        insolation.flux,
        atmosphere.albedo,
    );

    let mu = mean_molecular_weight(composition);

    let h = scale_height(
        temperature,
        mu,
        gravity,
    );

    let pressure = pressure_at_altitude(
        atmosphere.surface_pressure,
        h,
        surface.height,
    );

    EnvironmentConditions {
        temperature,
        pressure,
        insolation: insolation.flux,
        gravity,
    }
}
