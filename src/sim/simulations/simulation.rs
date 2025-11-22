use crate::sim::world::WorldState;
use crate::core::chronovox::{ChronoEvent, EventKind};

use uuid::Uuid;
use chrono::{Duration, Utc};

use crate::sim::entities::SimEntity;
use crate::core::objex::core::MaterialLink;

use crate::sim::components::Velocity;
use crate::sim::systems::{
    System,
    MovementSystem,
    AccelerationSystem,
    CollisionSystem,
    GravitySystem,
    FractureSystem,
    MassSystem,
    MechanicalSystem,
    StrengthSystem,
    ThermalSystem,
    ElectricalSystem,
    OpticalSystem,
    DegradationSystem,
};

use crate::sim::clock::SimClock;
use crate::core::tdt::sim_time::SimTime;
use crate::supabasic::worlds::WorldRecord;
use crate::sim::world::state::World; // runtime world
        use crate::core::objex::core::Objex;
        use crate::core::objex::geospec::shapes::Shape;
        use crate::core::uvoxid::UvoxId;

pub struct Simulation {
    pub simulation_id: Uuid,
    pub world_id: i64,

    pub sim_time: SimTime,
    pub clock: SimClock,

    pub world: WorldState,
    pub systems: Vec<Box<dyn System + Send + Sync>>,
    pub timeline: Vec<ChronoEvent>,
}


impl Simulation {

    /// Create a fresh simulation from DB world metadata.
    pub fn new(meta: WorldRecord) -> Self {

        // Convert persistent DB record → runtime world struct
        let runtime_world: World = meta.clone().into();

        let mut world_state = WorldState::new(runtime_world);

        //
        // EXAMPLE BOOTSTRAP ENTITY
        //
        let boot_id = Uuid::new_v4();

        // minimal material link
        let mat = MaterialLink::vacuum();


        // minimal test entity


        let shape = Shape::default_box();// adjust if different variants exist
        let blueprint = Objex {
            shape,
            material: MaterialLink::vacuum(),
                };

        let boot = SimEntity::spawn(
            blueprint,
            world_state.meta.world_id,
            UvoxId::new(0,0,0),      // or any position you want
            world_state.sim_time,  // must be valid SimTime
        );


        world_state.entities.insert(boot_id, boot);

        // give it some velocity
        world_state
            .components
            .velocity_components
            .insert(boot_id, Velocity::new(1.0, 0.0, 0.0));

        //
        // SYSTEM LIST
        //
        let systems: Vec<Box<dyn System + Send + Sync>> = vec![
            Box::new(GravitySystem),
            Box::new(AccelerationSystem),
            Box::new(MovementSystem),
            Box::new(CollisionSystem),
            Box::new(FractureSystem),
            Box::new(DegradationSystem),
            Box::new(MassSystem),
            Box::new(MechanicalSystem),
            Box::new(StrengthSystem),
            Box::new(ThermalSystem),
            Box::new(ElectricalSystem),
            Box::new(OpticalSystem),
        ];

        //
        // SIM CLOCK
        //
        let now = Utc::now();
        let start = now - Duration::days(365 * 50);
        let step  = Duration::days(30);

        let clock = SimClock::from_wall_dates(start, now, step);
        let sim_time = clock.current;

        Self {
            simulation_id: Uuid::new_v4(),
            world_id: world_state.meta.world_id,
            world: world_state,
            systems,
            sim_time,
            clock,
            timeline: Vec::new(),
        }
    }


    pub fn tick(&mut self) -> Vec<ChronoEvent> {

        // Advance simulation clock
        if !self.clock.advance() {
            tracing::info!("⏹ simulation end reached");
            return vec![];
        }

        self.sim_time = self.clock.current;

        // Push time into world state
        self.world.sim_time = self.sim_time;
        self.world.sim_delta = self.clock.step;
        self.world.clock = Some(self.clock.clone());

        let mut all_events = Vec::new();
        let t = self.sim_time;

        // Run all ECS systems
        for system in &mut self.systems {
            let mut evs = system.tick(&mut self.world);
            all_events.append(&mut evs);
        }

        // Guarantee at least one event per tick
        if all_events.is_empty() {
            all_events.push(ChronoEvent {
                entity_id: Uuid::nil(),
                world_id: self.world_id,
                t,
                kind: EventKind::Custom("EmptyTick".into()),
                payload: None,
            });
        }

        self.timeline.extend(all_events.clone());
        all_events
    }
}
