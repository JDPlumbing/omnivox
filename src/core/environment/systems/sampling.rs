use crate::core::environment::state::EnvironmentState;
use crate::core::environment::systems::surface::environment_at_surface;
use crate::core::entity::systems::declare_entity_environment_sample::declare_entity_environment_sample;
use crate::core::entity::id::EntityId;
use crate::shared::entities::entity_store::EntityStore;

use crate::core::worlds::state::WorldState;
use crate::core::worlds::systems::surface::sample_world_surface;
use crate::core::worlds::systems::gravity::gravity_enu_at_location;
use crate::core::worlds::systems::radiation::radiation_at_surface;

use crate::core::cosmic::state::CosmicState;
use crate::core::tdt::SimTime;

use crate::core::spatial::resolve::uvox_to_surface;
use crate::core::physics::units::acceleration::MetersPerSecondSquared;

/// Samples environment + gravity for all active entities.
///
/// This system:
/// - reads world + cosmic + environment descriptors
/// - computes per-entity environment conditions
/// - writes EntityEnvironmentSample
///
/// It does NOT:
/// - move entities
/// - apply forces
/// - advance time
pub fn sample_environment_for_active_entities(
    time: SimTime,
    cosmic: &CosmicState,
    world: &WorldState,
    environment: &EnvironmentState,
    entities: &mut EntityStore,
) {
    let active_entities: Vec<EntityId> =
        entities.actives.keys().cloned().collect();

    for entity in active_entities {
        let membership = match entities.world_memberships.get(&entity) {
            Some(m) => m,
            None => continue,
        };

        let pos_uvox = match entities.positions.get(&entity) {
            Some(p) => p.0,
            None => continue,
        };

        // --- Surface coordinates ---
        let surface_coords = uvox_to_surface(
            membership.world_id,
            pos_uvox,
            world,
            cosmic,
        );

        // --- World geometry ---
        let surface_sample = sample_world_surface(
            membership.world_id,
            &surface_coords,
            world,
        );

        // --- Gravity (vector) ---
        let gravity_local = gravity_enu_at_location(
            membership.world_id,
            &surface_coords,
            world,
            cosmic,
            time,
        );

        // Convert to scalar magnitude for environment
        let gravity_mag =
            MetersPerSecondSquared(-gravity_local.up.0);

        // --- Radiation ---
        let radiation = match radiation_at_surface(
            membership.world_id,
            world,
            cosmic,
            time,
        ) {
            Some(r) => r,
            None => continue, // no star / deep night
        };

        // --- Environment ---
        let env = environment_at_surface(
            membership.world_id,
            &surface_sample,
            &radiation,
            gravity_mag,
            environment,
        );

        // --- Store result ---
        declare_entity_environment_sample(
            entity,
            env,
            gravity_local,
            entities,
        );
    }
}
