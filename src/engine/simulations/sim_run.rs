// sim/sim_run.rs
use crate::engine::{
    world::WorldState,
    systems::System,
    clock::SimClock,
};
//use crate::core::tdt::sim_time::SimTime;
use crate::core::chronovox::ChronoEvent;

use chrono::{DateTime, Utc};
use serde_json::{Value, json};

/// Run a simulation between two UTC wall times.
///
/// step_ns: how many nanoseconds to advance per tick.
pub fn run_simulation_between(
    world: &mut WorldState,
    systems: &mut [Box<dyn System>],
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    step_ns: i128,
) -> Vec<Value> {

    let mut log = Vec::new();
    let mut clock = SimClock::from_utc_range(start, end, step_ns);

    while clock.current_ns() < clock.end.as_ns() {
        world.clock = Some(clock.clone());

        let mut tick_events: Vec<ChronoEvent> = Vec::new();

        for sys in systems.iter_mut() {
            let evs = sys.tick(world);
            if !evs.is_empty() {
                tick_events.extend(evs);
            }
        }

        if !tick_events.is_empty() {
            log.push(json!({
                "tick_time_ns": clock.current_ns(),
                "events": tick_events,
            }));
        }

        clock.advance();
    }

    log
}
