use crate::shared::entities::entity_store::EntityStore;
use crate::core::worlds::systems::surface::sample_world_surface;
use crate::core::spatial::surface::SurfaceCoords;
use crate::core::physics::units::length::Meters;
use crate::core::entity::components::grounded::Grounded;
use crate::core::physics::units::velocity::MetersPerSecond;

pub fn apply_ground_constraint(
    store: &mut EntityStore,
    world_state: &crate::core::worlds::state::WorldState,
) {
    let active_entities: Vec<_> = store.actives.keys().copied().collect();

    for entity in active_entities {
        let Some(pos) = store.position_enus.get_mut(&entity) else { continue };
        let Some(world) = store.world_memberships.get(&entity) else { continue };

        // For now: assume entity surface coords already exist or are trivial
        // (you can later replace this with Uvox / spatial resolution)
        let surface_coords = SurfaceCoords {
            latitude: crate::core::physics::units::angle::Radians(0.0),
            longitude: crate::core::physics::units::angle::Radians(0.0),
            elevation: Meters(0.0),
        };

        let surface = sample_world_surface(
            world.world_id,
            &surface_coords,
            world_state,
        );

        let ground_height = surface.height.0;

        if pos.up.0 < ground_height {
                // --- Constraint active ---
                pos.up = Meters(ground_height);

                if let Some(vel) = store.velocity_enus.get_mut(&entity) {
                    vel.up = MetersPerSecond(0.0);
                }

                // Mark grounded
                store.groundeds.insert(entity, Grounded);
            } else {
                // Not grounded
                store.groundeds.remove(&entity);
            }

    }
}
