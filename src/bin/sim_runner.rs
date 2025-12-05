use chrono::{Utc};
use omnivox::{
    core::{
        id::WorldId,
        objex::core::material::MaterialLink,
        objex::Objex,
        tdt::sim_time::SimTime,
        tdt::sim_duration::SimDuration,
        uvoxid::UvoxId,
    },
    sim::{
        entities::SimEntity,
        simulations::simulation::Simulation,
        systems::movement::MovementSystem,
        world::state::{World, WorldState},
        components::velocity::Velocity,
        time::clock::SimClock,
        UvoxQuat,
    },
};

fn main() {
    println!("--- Sim Runner Starting ---");

    //
    // 1) Create a World (NOT a DB world record)
    //
    let world_meta = World {
        id: WorldId::from(0),
        name: Some("Earth".into()),
        description: Some("Local baseline world".into()),
        world_epoch: Some(SimTime::from_ns(0)),
    };

    //
    // 2) Build runtime state
    //
    let mut world_state = WorldState::new(world_meta.clone());

    //
    // 3) Spawn one entity
    //
    let eid = world_state.allocate_entity_id();

    let material = MaterialLink::vacuum();
    let obj = Objex::sphere(material, 1.0);

    let pos = UvoxId::earth(6371000000000, 0, 0);

    let ent = SimEntity::spawn(
        eid,
        obj,
        world_meta.id,
        pos,
        UvoxQuat::identity(),
        SimTime::from_ns(0),
    );

    world_state.entities.insert(eid, ent);

    //
    // 4) Give the entity velocity so it moves
    //
    world_state
        .components
        .velocity_components
        .insert(eid, Velocity::new(10.0, 0.0, 0.0));

    //
    // 5) Make a SimClock
    //
    let clock = SimClock::fixed_step(
        SimTime::from_ns(0),
        1_000_000_000, // 1 sec per tick
    );

    //
    // 6) Build Simulation instance
    //
    let mut sim = Simulation {
        simulation_id: Default::default(),
        world_id: world_meta.id,
        sim_time: SimTime::from_ns(0),
        clock,
        world: world_state,
        systems: vec![Box::new(MovementSystem {})],
        timeline: vec![],
    };

    println!("World loaded: {:?}", world_meta.name);

    //
    // 7) Run 5 ticks
    //
    for step in 0..5 {
        let evs = sim.tick();

        println!(
            "Tick {} → sim_time {:?}, events = {}",
            step, sim.sim_time, evs.len()
        );

        for (id, ent) in &sim.world.entities {
            println!("   {} -> {:?}", id, ent.position);
        }
    }

    println!("--- Sim Runner Complete ---");
}
