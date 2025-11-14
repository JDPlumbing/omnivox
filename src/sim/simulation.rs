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
use crate::tdt::sim_time::{ SimTime, SimDuration };

pub struct Simulation {
    pub simulation_id: Uuid,
    pub frame_id: i64,

    pub sim_time: SimTime,
    pub clock: SimClock,

    pub timeline: Vec<ChronoEvent>,
    pub world: WorldState,
    pub systems: Vec<Box<dyn System + Send + Sync>>,
}

impl Simulation {
    pub fn new(meta_world: WorldRow) -> Self {
        let mut world_state = WorldState::new(meta_world);

        // TEMP example object:
        let test_id = uuid::Uuid::new_v4();
        world_state.objects.insert(
            test_id.to_string(),
            Objex::new_box(0, None, MaterialLink::vacuum(), 1.0, 1.0, 1.0),
        );
        world_state
            .velocity_components
            .insert(test_id, Velocity::new(1.0, 0.0, 0.0));

        // Systems list (your existing list is fine)
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
            Box::new(DegradationSystem),
            Box::new(OpticalSystem),
            Box::new(CompositeSystem),
        ];

        // New clock based on wall time
        let now = Utc::now();
        let start = now - Duration::days(365 * 50); // 50 years ago
        let step = Duration::days(30);              // 1 simulated month per tick

        let clock = SimClock::from_wall_dates(start, now, step);

        Self {
            simulation_id: Uuid::new_v4(),
            frame_id: world_state.meta.frame_id,
            world: world_state,
            timeline: Vec::new(),
            systems,
            
            sim_time: SimTime::from_ns(clock.current_ns),
            clock,
        }
    }



    pub fn tick(&mut self) -> Vec<ChronoEvent> {
        if !self.clock.advance() {
            tracing::info!("⏹ Simulation reached end date");
            return vec![];
        }

        // NEW: update absolute sim time
        self.sim_time = SimTime::from_ns(self.clock.current_ns);

        let mut all_events = Vec::new();

        for sys in &mut self.systems {
            let events = sys.tick(&mut self.world);
            all_events.extend(events);
        }

        // OPTIONAL: keep this fallback if needed
        if all_events.is_empty() {
            all_events.push(ChronoEvent {
                id: UvoxId::new(0, 0, 0, 0),
                t: TimeDelta::from_sim_duration(
                    SimDuration::from_ns(self.clock.step_ns)
                ),
                kind: EventKind::Custom("EmptyTick".into()),
                payload: None,
            });
        }

        self.timeline.extend(all_events.clone());
        all_events
    }


}
