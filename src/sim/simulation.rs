use crate::sim::world::World;
use crate::chronovox::ChronoEvent;
use crate::sim::systems::System;
use std::collections::HashMap;
use uuid::Uuid;

/// A live simulation instance in memory
pub struct Simulation {
    pub simulation_id: Uuid,
    pub current_tick: i64,
    pub frame_id: i64,
    pub timeline: Vec<ChronoEvent>,
    pub world: World,
    pub systems: Vec<Box<dyn System + Send>>,
    
}

impl Simulation {
    pub fn new(world: World, systems: Vec<Box<dyn System + Send>>) -> Self {
        Simulation {
            simulation_id: Uuid::new_v4(),
            current_tick: 0,
            frame_id: 0,
            world,
            timeline: Vec::new(),
            systems,
        }
    }

    /// Advance the simulation by one tick
    pub fn tick(&mut self) -> Vec<ChronoEvent> {
        self.current_tick += 1;

        let mut all_events = Vec::new();
        for sys in &mut self.systems {
            let events = sys.run(&mut self.world);
            all_events.extend(events);
        }

        // Append to timeline
        self.timeline.extend(all_events.clone());

        all_events
    }
}
