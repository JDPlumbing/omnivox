use crate::core::environment::conditions::EnvironmentConditions;
use crate::core::worlds::systems::insolation::insolation_at_surface;
use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;
use crate::core::cosmic::state::CosmicState;
use crate::core::spatial::surface::SurfaceCoords;
use crate::core::tdt::sim_time::SimTime;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;

pub fn environment_at_surface(
    world_id: WorldId,
    location: &SurfaceCoords,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
    time: SimTime,
) -> EnvironmentConditions {
    // Start with defaults (pressure, gravity, baseline temp)
    let mut env = EnvironmentConditions::default();

    // --- Insolation (world → cosmic → time)
    if let Some(insolation) = insolation_at_surface(
        world_id,
        location,
        world_state,
        cosmic_state,
        time,
    ) {
        env.insolation = insolation.flux;
    } else {
        env.insolation = WattsPerSquareMeter(0.0);
    }

    env
}
