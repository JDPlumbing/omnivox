use crate::core::simulation::engine::SimulationEngine;
use crate::core::tdt::sim_time::SimTime;
use crate::core::entity::components::spatial::{
    position_enu::PositionENU,
    velocity_enu::VelocityENU,
    world_membership::WorldMembership,
};
use crate::core::worlds::id::WorldId;
use crate::shared::entities::entity_store::EntityStore;
use crate::engine::entity::entity_engine::EntityEngine;
use crate::core::entity::components::meta::note::Note;

#[test]
fn entity_moves_linearly_over_time() {
    // 1. Setup simulation
    let mut store = EntityStore::default();
    let mut entity_engine = EntityEngine::new(&mut store);

   

let entity = entity_engine.create_note_entity(Note{text:"test".to_string()});

    entity_engine.set_world(entity, WorldId(1));

    store.position_enus.insert(entity, PositionENU {
        east: 0.0,
        north: 0.0,
        up: 0.0,
    });

    store.velocity_enus.insert(entity, VelocityENU {
        ve: 1.0,   // 1 m/s east
        vn: 0.0,
        vu: 0.0,
    });

    store.actives.insert(entity, crate::core::entity::components::meta::active::Active);

    let mut sim = SimulationEngine::new(
        SimTime::from_ns(0),
        1_000_000_000, // 1 second per tick
        store,
    );

    // 2. Run 10 ticks
    for _ in 0..10 {
        sim.tick();
    }

    // 3. Assert final position
    let pos = sim.state.entities.position_enus.get(&entity).unwrap();

    assert_eq!(pos.east, 10.0);
    assert_eq!(pos.north, 0.0);
    assert_eq!(pos.up, 0.0);
}
