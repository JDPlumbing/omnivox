use crate::sim::world::World;
use crate::chronovox::ChronoEvent;
use crate::sim::systems::System;
use std::collections::HashMap;
use uuid::Uuid;
use crate::uvoxid::UvoxId;
use crate::chronovox::EventKind;
use crate::tdt::core::TimeDelta;

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
        let mut sim = Simulation {
            simulation_id: Uuid::new_v4(),
            current_tick: 0,
            frame_id: 0,
            world,
            timeline: Vec::new(),
            systems,
        };

        // TEMP: add a movement event to see your MovementSystem do something
        sim.timeline.push(ChronoEvent {
            id: UvoxId::new(0, 0, 0, 0),
            t: TimeDelta::from_ticks(0, "nanoseconds"),
            kind: EventKind::Move { dr: 10, dlat: 0, dlon: 0 },
            payload: None,
        });

        sim
    }

    /// Advance the simulation by one tick
    pub fn tick(&mut self) -> Vec<ChronoEvent> {
        self.current_tick += 1;
        let mut all_events = Vec::new();

        // Temporarily borrow only the world
        let world_ptr: *mut _ = &mut self.world;

        for sys in &mut self.systems {
            // SAFETY: Each system runs sequentially, not in parallel.
            // So it's safe to mutably borrow world one at a time.
            let events = unsafe { sys.tick(&mut *world_ptr) };
            all_events.extend(events);
        }

        self.timeline.extend(all_events.clone());
        all_events
    }

}
