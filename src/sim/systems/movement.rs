use crate::sim::world::World;
use crate::chronovox::{ChronoEvent, EventKind, UvoxId};
use crate::sim::systems::System;
use crate::tdt::core::TimeDelta;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, world: &mut World) -> Vec<ChronoEvent> {
        let mut applied = Vec::new();
        let tick = world.current_tick;

        for ev in world.timeline.events.iter().filter(|e| e.t.ticks("nanoseconds") == tick) {
            if let EventKind::Move { dr, dlat, dlon } = ev.kind {
                if let Some(obj) = world.objects.remove(&ev.id) {
                    let mut new_id = ev.id.clone();

                    // Apply spherical deltas with i64 math
                    let new_r = (new_id.r_um as i64) + dr;
                    if new_r < 0 {
                        new_id.r_um = 0;
                    } else {
                        new_id.r_um = new_r as u64;
                    }

                    new_id.lat_code += dlat;
                    new_id.lon_code += dlon;

                    world.objects.insert(new_id.clone(), obj);

                    let new_ev = ChronoEvent {
                        id: new_id,
                        t: TimeDelta::from_ticks(world.current_tick, "nanoseconds"),
                        kind: ev.kind.clone(),
                        payload: None,
                    };

                    applied.push(new_ev);
                }
            }

        }

        applied
    }
}
