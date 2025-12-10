use chrono::{Duration, Utc};

use crate::core::id::{WorldId, SimulationId, UserId};
use crate::core::id::UvoxRegionId;

use crate::core::objex::core::{MaterialLink, Objex};
use crate::core::objex::geospec::shapes::Shape;

use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

use crate::core::uvoxid::{UvoxId, LatCode, LonCode};
use crate::sim::entities::UvoxQuat;

use crate::sim::clock::SimClock;
use crate::sim::components::Velocity;
use crate::sim::entities::SimEntity;
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

use crate::supabasic::worlds::WorldRow;
use crate::sim::simulations::simulation_config::SimulationConfig;
use crate::sim::simulations::persist::state::PersistedSimState;


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


// ============================================================================
// IMPL STARTS HERE — everything MUST be inside this block.
// ============================================================================
impl Simulation {

    /// Construct a new simulation from the DB world metadata.
    pub fn new(meta: WorldRow) -> Self {
        let metadata_world: World = meta.clone().into();
        let world_id = metadata_world.id;

        let mut world_state = WorldState::new(metadata_world.clone());

        // test entity
        let boot_id: EntityId = world_state.allocate_entity_id();
        let blueprint = Objex {
            shape: Shape::default_box(),
            material: MaterialLink::vacuum(),
        };

        let initial_pos = UvoxId::earth_surface(LatCode(0), LonCode(0));
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

        world_state.components.velocity_components.insert(
            boot_id,
            Velocity::new(1.0, 0.0, 0.0),
        );

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

        let now = Utc::now();
        let start = now - Duration::days(365 * 50);
        let step = Duration::days(30);

        let clock = SimClock::from_wall_dates(start, now, step);
        let sim_time = clock.current;

        let simulation_id = SimulationId::new(
            world_id,
            UvoxRegionId::default(),
            sim_time,
            UserId::zero(),
            0,
        );

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


    // ========================================================================
    // NEW: new_from_config
    // ========================================================================
    pub fn new_from_config(cfg: &SimulationConfig, world_record: WorldRow) -> Self {
        let metadata_world: World = world_record.clone().into();
        let world_id = metadata_world.id;

        let mut world_state = WorldState::new(metadata_world);
        world_state.sim_time = cfg.start_time;
        world_state.sim_delta = SimDuration::from_seconds(1);

        let boot_id = world_state.allocate_entity_id();
        let blueprint = Objex {
            shape: Shape::default_box(),
            material: MaterialLink::vacuum(),
        };

        let initial_pos = cfg.region.min;
        let initial_orientation = UvoxQuat::identity();

        let boot_entity = SimEntity::spawn(
            boot_id,
            blueprint,
            world_id,
            initial_pos,
            initial_orientation,
            cfg.start_time,
        );

        world_state.entities.insert(boot_id, boot_entity);

        world_state.components.velocity_components.insert(
            boot_id,
            Velocity::new(1.0, 0.0, 0.0),
        );

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

        let clock = SimClock::from_wall_dates(
            Utc::now(),
            Utc::now() + Duration::days(1),
            Duration::seconds(1),
        );

        Self {
            simulation_id: SimulationId::new(
                cfg.world_id,
                cfg.region,
                cfg.start_time,
                cfg.user_id,
                cfg.branch,
            ),
            world_id,
            sim_time: cfg.start_time,
            clock,
            world: world_state,
            systems,
            timeline: Vec::new(),
        }
    }


    // ========================================================================
    // NEW: from_persisted
    // ========================================================================
    pub fn from_persisted(
        world_record: WorldRow,
        persisted: PersistedSimState,
        cfg: SimulationConfig,
    ) -> Self {
        let metadata_world: World = world_record.clone().into();
        let world_id = metadata_world.id;

        let mut world_state = WorldState::new(metadata_world);
        world_state.sim_time = cfg.start_time;
        world_state.sim_delta = SimDuration::from_seconds(1);

        // TODO: apply persisted.entities, persisted.components

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

        let clock = SimClock::from_wall_dates(
            Utc::now(),
            Utc::now() + Duration::hours(1),
            Duration::seconds(1),
        );

        Self {
            simulation_id: SimulationId::new(
                cfg.world_id,
                cfg.region,
                cfg.start_time,
                cfg.user_id,
                cfg.branch,
            ),
            world_id,
            sim_time: cfg.start_time,
            clock,
            world: world_state,
            systems,
            timeline: Vec::new(),
        }
    }


    // ========================================================================
    // Advance simulation
    // ========================================================================
    pub fn tick(&mut self) -> Vec<ChronoEvent> {
        if !self.clock.advance() {
            tracing::info!("⏹ simulation end reached");
            return vec![];
        }

        self.sim_time = self.clock.current;
        self.world.sim_time = self.sim_time;
        self.world.sim_delta = self.clock.step;
        self.world.clock = Some(self.clock.clone());

        let mut events = Vec::new();
        for system in &mut self.systems {
            let mut evs = system.tick(&mut self.world);
            events.append(&mut evs);
        }

        if events.is_empty() {
            events.push(ChronoEvent {
                entity_id: EntityId::new(0, 0),
                world_id: self.world_id,
                t: self.sim_time,
                kind: EventKind::Custom("EmptyTick".into()),
                payload: None,
            });
        }

        self.timeline.extend(events.clone());
        events
    }
}

/*
    pub fn simulate_to(&mut self, target: SimTime) -> Vec<ChronoEvent> {
        let mut events = vec![];

        while self.sim_time < target {
            let dt = target - self.sim_time;

            // 1. Scan systems for predicted events
            let mut predicted = vec![];

            for sys in &self.systems {
                if let Some((tevent, kind, entity)) = sys.predict_event(&self.world, dt) {
                    predicted.push((tevent, kind, entity, sys.as_ref()));
                }
            }

            // 2. No events → final analytic jump
            if predicted.is_empty() {
                for sys in &mut self.systems {
                    sys.apply_analytical(&mut self.world, dt);
                }
                self.sim_time = target;
                break;
            }

            // 3. Earliest event
            predicted.sort_by_key(|p| p.0);
            let (tevent, kind, entity, system_ref) = predicted[0].clone();

            // 4. Jump to event time
            for sys in &mut self.systems {
                sys.apply_analytical(&mut self.world, tevent);
            }
            self.sim_time += tevent;

            // 5. Emit event
            let ce = ChronoEvent::new(entity, self.world.world_id, self.sim_time, kind.clone());
            events.push(ce);

            // 6. Apply system-specific event actions
            system_ref.apply_event(&mut self.world, entity, &kind);
        }

        events
    }
        
}
impl Simulation {
    pub fn add_system<S: AnalyticalSystem + 'static>(&mut self, sys: S) {
        self.systems.push(Box::new(sys));
    }
}
*/