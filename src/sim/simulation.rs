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
use crate::sim::systems::{System, MovementSystem, AccelerationSystem, CollisionSystem};

pub struct Simulation {
    pub simulation_id: Uuid,
    pub current_tick: i64,
    pub frame_id: i64,
    pub timeline: Vec<ChronoEvent>,
    pub world: WorldState,
    pub systems: Vec<Box<dyn System + Send + Sync>>,
}


impl Simulation {
    pub fn new(meta_world: WorldRow) -> Self {
        let mut world_state = WorldState::new(meta_world);

        let test_id = uuid::Uuid::new_v4();
        world_state.objects.insert(
            test_id.to_string(),
            Objex::new_box(0, None, MaterialLink::vacuum(), 1.0, 1.0, 1.0),
        );

        world_state.velocity_components.insert(test_id, Velocity::new(1.0, 0.0, 0.0));
        world_state.acceleration_components.insert(test_id, Acceleration::new(0.0, -9.81, 0.0));

        tracing::info!(
            "🧩 Created test object: {:?}\nVelocity keys: {:?}\nAcceleration keys: {:?}",
            test_id,
            world_state.velocity_components.keys().collect::<Vec<_>>(),
            world_state.acceleration_components.keys().collect::<Vec<_>>()
        );


        // ✅ Define systems here
        let systems: Vec<Box<dyn System + Send + Sync>> = vec![
            Box::new(AccelerationSystem),
            Box::new(MovementSystem),
            Box::new(CollisionSystem),
        ];


        Self {
            simulation_id: Uuid::new_v4(),
            current_tick: 0,
            frame_id: world_state.meta.frame_id,
            world: world_state,
            timeline: Vec::new(),
            systems,
        }
    }


pub fn tick(&mut self) -> Vec<ChronoEvent> {
    // Advance logical clock by one frame
    self.current_tick += 1;

    tracing::info!("🔄 Tick {} for simulation {}", self.current_tick, self.simulation_id);

    // Collect new events only from this frame
    let mut all_events = Vec::new();

    // Run every active system exactly once per tick
    for sys in &mut self.systems {
        let events = sys.tick(&mut self.world);
        all_events.extend(events);
    }

    // ✅ Safety: If no systems are registered, create a single dummy movement event
    if all_events.is_empty() {
        all_events.push(ChronoEvent {
            id: UvoxId::new(0, 0, 0, 0),
            t: TimeDelta::from_ticks(self.current_tick, "nanoseconds"),
            kind: EventKind::Move { dr: 1, dlat: 0, dlon: 0 },
            payload: None,
        });
    }

    // Record what happened this tick into the timeline
    self.timeline.extend(all_events.clone());

    // ✅ Return just this tick’s events — nothing here should re-trigger or loop
    all_events
}


}
