use chrono::{Duration, Utc};

use crate::core::id::{WorldId, SimulationId, UserId};
use crate::core::id::UvoxRegionId;

use crate::core::objex::core::{MaterialLink, Objex};
use crate::core::objex::geospec::shapes::Shape;

use crate::core::tdt::sim_time::SimTime;
use crate::core::uvoxid::{UvoxId};
use crate::sim::entities::UvoxQuat;

use crate::sim::clock::SimClock;
use crate::sim::components::Velocity;
use crate::sim::entities::{SimEntity};
use crate::sim::world::state::{WorldState, World};

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
    ElectricalSystem,
    OpticalSystem,
    DegradationSystem,
};
use crate::sim::systems::solar_exposure::SolarExposureSystem;

use crate::core::chronovox::{ChronoEvent, EventKind};
use crate::core::id::entity_id::EntityId;

use crate::supabasic::worlds::WorldRecord;

/// ---------------------------------------------------------------------------
/// Simulation — runtime ECS + clock + systems
/// ---------------------------------------------------------------------------
pub struct Simulation {
    pub simulation_id: SimulationId,
    pub world_id: WorldId,

    pub sim_time: SimTime,
    pub clock: SimClock,

    pub world: WorldState,
    pub systems: Vec<Box<dyn System + Send + Sync>>,
    pub timeline: Vec<ChronoEvent>,
}

impl Simulation {

    /// Construct a new simulation from the DB world metadata.
    pub fn new(meta: WorldRecord) -> Self {

        //
        // 1. Convert DB → metadata World
        //
        let metadata_world: World = meta.clone().into();
        let world_id = metadata_world.id;

        //
        // 2. Runtime world state
        //
        let mut world_state = WorldState::new(metadata_world.clone());

        //
        // 3. Create a bootstrapped test entity
        //
        let boot_id: EntityId = world_state.allocate_entity_id();

        let blueprint = Objex {
            shape: Shape::default_box(),
            material: MaterialLink::vacuum(),
        };

        let initial_pos = UvoxId::new(0,0,0);
        let initial_orientation = UvoxQuat::identity();

        let boot_entity = SimEntity::spawn(
            boot_id,
            blueprint,
            world_id,
            initial_pos,
            initial_orientation,
            world_state.sim_time,
        );

        world_state.entities.insert(boot_id, boot_entity);

        // Give velocity
        world_state.components.velocity_components.insert(
            boot_id,
            Velocity::new(1.0, 0.0, 0.0),
        );

        //
        // 4. Install all systems
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
            Box::new(ElectricalSystem),
            Box::new(OpticalSystem),
            Box::new(SolarExposureSystem),
        ];

        //
        // 5. Build simulation clock
        //
        let now = Utc::now();
        let start = now - Duration::days(365 * 50);
        let step = Duration::days(30);

        let clock = SimClock::from_wall_dates(start, now, step);
        let sim_time = clock.current;

        //
        // 6. Build typed simulation ID (Option A)
        //
        let simulation_id = SimulationId::new(
            world_id,
            UvoxRegionId::default(),
            sim_time,
            UserId::from(0),
            0, // branch
        );

        // 7. Construct simulation object
        //
        Self {
            simulation_id,
            world_id,
            world: world_state,
            systems,
            sim_time,
            clock,
            timeline: Vec::new(),
        }
    }

    /// Advance simulation a single step
    pub fn tick(&mut self) -> Vec<ChronoEvent> {

        // Advance internal clock
        if !self.clock.advance() {
            tracing::info!("⏹ simulation end reached");
            return vec![];
        }

        self.sim_time = self.clock.current;

        // Sync world timing
        self.world.sim_time = self.sim_time;
        self.world.sim_delta = self.clock.step;
        self.world.clock = Some(self.clock.clone());

        let mut all_events = Vec::new();

        //
        // Run all systems
        //
        for system in &mut self.systems {
            let mut evs = system.tick(&mut self.world);
            all_events.append(&mut evs);
        }

        //
        // Always produce at least one event
        //
        if all_events.is_empty() {
            all_events.push(ChronoEvent {
                entity_id: EntityId::new(0, 0),
                world_id: self.world_id,
                t: self.sim_time,
                kind: EventKind::Custom("EmptyTick".into()),
                payload: None,
            });
        }

        self.timeline.extend(all_events.clone());
        all_events
    }
}
