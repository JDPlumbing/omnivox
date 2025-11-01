use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::systems::System,
    sim::world::World,
    tdt::core::TimeDelta,
};

/// Simple movement system that updates object positions based on `Move` events
pub struct MovementSystem;

impl System for MovementSystem {
    fn name(&self) -> &'static str {
        "MovementSystem"
    }

    fn tick(&mut self, world: &mut World) -> Vec<ChronoEvent> {
        let mut triggered_events = Vec::new();

        // For now, fake a tick counter if we don’t yet track it in `World`
        let tick = 0_i64;

        // Example placeholder — replace later with world.timeline or similar
        let active_events: Vec<ChronoEvent> = Vec::new();

        for ev in active_events.iter().filter(|e| e.t.ticks("nanoseconds") == tick) {
            if let EventKind::Move { dr, dlat, dlon } = ev.kind {
                let mut new_id = ev.id.clone();

                new_id.r_um = (new_id.r_um + dr).max(0);
                new_id.lat_code += dlat;
                new_id.lon_code += dlon;

                triggered_events.push(ChronoEvent {
                    id: new_id,
                    t: TimeDelta::from_ticks(1, "nanoseconds"),

                    kind: ev.kind.clone(),
                    payload: None,
                });
            }
        }

        triggered_events
    }

}
