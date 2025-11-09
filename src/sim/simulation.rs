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

pub struct Simulation {
    pub simulation_id: Uuid,
    pub current_tick: i64,
    pub frame_id: i64,
    pub timeline: Vec<ChronoEvent>,
    pub world: WorldState,
    pub systems: Vec<Box<dyn System + Send + Sync>>,
    pub clock: SimClock,
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

        tracing::info!(
            "🧩 Created test object: {:?}\nVelocity keys: {:?}\nAcceleration keys: {:?}",
            test_id,
            world_state.velocity_components.keys().collect::<Vec<_>>(),
            world_state.acceleration_components.keys().collect::<Vec<_>>()
        );
        if let Some(obj) = world_state.objects.get(&test_id.to_string()) {
        if let Some(props) = obj.material.props() {
            tracing::info!(
                "🧪 Material properties for {:?}: density = {:.2}, elastic_modulus = {:.2}, hardness = {:.2}",
                obj.material.name,
                props.density,
                props.elastic_modulus,
                props.hardness
            );
        } else {
            tracing::info!("⚠️ No matcat properties found for {:?}", obj.material.name);
        }
    }

        // ✅ Define systems here
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

        let now = Utc::now();
        let start = now - Duration::days(365 * 50); // e.g. 50 years ago placeholder
        let step = Duration::days(30); // 1 month per tick
        let clock = SimClock::new(start, now, step);

        Self {
            simulation_id: Uuid::new_v4(),
            current_tick: 0,
            frame_id: world_state.meta.frame_id,
            world: world_state,
            timeline: Vec::new(),
            systems,
            clock,
        }
    }


pub fn tick(&mut self) -> Vec<ChronoEvent> {
    if !self.clock.advance() {
        tracing::info!("⏹ Simulation reached end date");
        return vec![];
    }

    self.current_tick += 1;
    let mut all_events = Vec::new();

    for sys in &mut self.systems {
        let events = sys.tick(&mut self.world);
        all_events.extend(events);
    }

    if all_events.is_empty() {
        all_events.push(ChronoEvent {
            id: UvoxId::new(0, 0, 0, 0),
            t: self.clock.now_delta(),
            kind: EventKind::Move { dr: 1, dlat: 0, dlon: 0 },
            payload: None,
        });
    }

    self.timeline.extend(all_events.clone());
    all_events
}


}
