// tests/common/mod.rs
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::components::{
    mass::Mass,
    radius::Radius,
    root::Root,
};
use crate::core::physics::units::{
    mass::Kilograms,
    length::Meters,
};
use crate::core::simulation::engine::SimulationEngine;
use crate::core::worlds::state::WorldState;
use crate::core::tdt::sim_time::SimTime;
use crate::core::worlds::id::WorldId;
use crate::core::cosmic::id::CosmicBodyId;
use crate::shared::entities::entity_store::EntityStore;
use crate::core::worlds::components::world_anchor::WorldAnchor;


pub fn test_simulation_with_earth(store: EntityStore) -> SimulationEngine {
    let earth_body_id = CosmicBodyId(1);
    let world_id = WorldId(1);

    // Cosmic
    let mut cosmic_state = CosmicState::default();
    cosmic_state.masses.insert(
        earth_body_id,
        Mass { kg: Kilograms(5.972e24) },
    );
    cosmic_state.radii.insert(
        earth_body_id,
        Radius { meters: Meters(6_371_000.0) },
    );
    cosmic_state.roots.insert(earth_body_id, Root);

    // World
    let mut world_state = WorldState::default();
    world_state.anchors.insert(
        world_id,
        WorldAnchor { body: earth_body_id },
    );

    SimulationEngine::new_with_state(
        SimTime::from_ns(0),
        1_000_000_000,
        cosmic_state,
        world_state,
        store,
    )

}
