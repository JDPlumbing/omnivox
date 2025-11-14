use crate::sim::world::WorldState;
use crate::chronovox::{ChronoEvent, EventKind};
use crate::uvoxid::UvoxId;
use crate::tdt::core::TimeDelta;
use uuid::Uuid;
use std::collections::HashMap;
use crate::objex::core::types::Objex;
use crate::supabasic::WorldRow;
use crate::objex::core::types::MaterialLink;
use crate::sim::components::{Velocity, Acceleration};
use crate::sim::systems::{  System, 
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
                            CompositeSystem, 
                            DegradationSystem};

use crate::sim::clock::SimClock;
use chrono::Duration;
use chrono::Utc;
use crate::tdt::sim_time::SimTime;
use crate::tdt::sim_duration::SimDuration;

pub struct Simulation {
    pub simulation_id: Uuid,
    pub frame_id: i64,

    // Sim time authority (absolute time cursor)
    pub sim_time: SimTime,

    // Time stepping controller (start/end/step)
    pub clock: SimClock,

    // Domain containers
    pub world: WorldState,
    pub systems: Vec<Box<dyn System + Send + Sync>>,
    pub timeline: Vec<ChronoEvent>,
}



impl Simulation {
    pub fn new(meta_world: WorldRow) -> Self {
        let mut world_state = WorldState::new(meta_world);

        // Example boot object (your same logic)
        let test_id = uuid::Uuid::new_v4();
        world_state.objects.insert(
            test_id.to_string(),
            Objex::new_box(0, None, MaterialLink::vacuum(), 1.0, 1.0, 1.0),
        );
        world_state.velocity_components.insert(
            test_id,
            Velocity::new(1.0, 0.0, 0.0),
        );

        // Systems list (unchanged)
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
            Box::new(CompositeSystem),
        ];

        // Unified time authority
        let now = Utc::now();
        let start = now - Duration::days(365 * 50);
        let step = Duration::days(30);

        let clock = SimClock::from_wall_dates(start, now, step);
        let sim_time = clock.current;


        Self {
            simulation_id: Uuid::new_v4(),
            frame_id: world_state.meta.frame_id,
            world: world_state,
            systems,
            timeline: Vec::new(),

            sim_time,
            clock,
        }
    }



    pub fn tick(&mut self) -> Vec<ChronoEvent> {

        // Advance clock; if we hit the end, stop
        let new_time = self.clock.advance();
            if new_time.is_none() {
                tracing::info!("⏹ Simulation reached end date");
                return vec![];
            }

            self.sim_time = new_time.unwrap();
        // Push time into world
        self.world.sim_time = self.sim_time;
        self.world.sim_delta = self.clock.step;

        self.world.clock = Some(self.clock.clone());

        let mut all_events = Vec::new();

        // Execute all systems
        for system in &mut self.systems {
            let mut events = system.tick(&mut self.world);
            all_events.append(&mut events);
        }

        // Keep the original “empty tick” fallback
        if all_events.is_empty() {
            all_events.push(ChronoEvent {
                id: UvoxId::new(0, 0, 0, 0),
                t: TimeDelta::from_sim_duration(
                    SimDuration::from_ns(self.clock.step_ns())
                ),
                kind: EventKind::Custom("EmptyTick".into()),
                payload: None,
            });
        }

        // Add to timeline
        self.timeline.extend(all_events.clone());
        all_events
    }
}
