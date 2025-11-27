// src/dev/scratch.rs
//
// Demonstrates:
//  - Generating fake ChronoEvents across simulated time
//  - Producing JSON aggregates using AggregateResolution
//  - Writing output to /dev JSON files for inspection

use omnivox::core::chronovox::{ChronoEvent, EventKind};
use omnivox::core::tdt::sim_time::SimTime;
use omnivox::core::tdt::sim_calendar::*;
use omnivox::core::tdt::sim_aggregate::{
    aggregate_events_json_pretty,
    AggregateResolution,
};

use uuid::Uuid;
use serde_json::json;
use std::fs;

fn main() {
    println!("▶ Running scratch time-aggregation demo…");

    //----------------------------------------------------------------------
    // 1. Generate test events across a span of simulated months/days
    //----------------------------------------------------------------------
    let mut events = Vec::new();

    let world_id = 1;
    let entity_a = Uuid::new_v4();
    let entity_b = Uuid::new_v4();

    // Produce events at different simulated times:
    //   month 0, month 1, month 2, etc.
    for month in 0..6 {
        let base_t = SimTime::from_ns(month * NANOS_PER_MONTH);

        // A: Installation event at the beginning of each month
        events.push(ChronoEvent {
            entity_id: entity_a,
            world_id,
            t: base_t,
            kind: EventKind::Custom(format!("Install_M{}", month)),
            payload: Some(json!({ "info": format!("EntityA installed month {}", month) })),
        });

        // B: Thermal update in the middle of the month
        let mid = base_t.add_days(15);
        events.push(ChronoEvent {
            entity_id: entity_b,
            world_id,
            t: mid,
            kind: EventKind::Custom(format!("Thermal_M{}", month)),
            payload: Some(json!({ "temp": 20.0 + month as f64 })),
        });
    }

    // A few daily events inside month 3
    let month3_start = SimTime::from_ns(3 * NANOS_PER_MONTH);
    for day in 0..5 {
        let t = month3_start.add_days(day);
        events.push(ChronoEvent {
            entity_id: entity_a,
            world_id,
            t,
            kind: EventKind::Custom(format!("DailyCheck_D{}", day)),
            payload: None,
        });
    }

    println!("Generated {} test events.", events.len());

    //----------------------------------------------------------------------
    // 2. Run aggregation at several resolutions
    //----------------------------------------------------------------------
    fs::create_dir_all("dev").unwrap();

    let raw_json   = aggregate_events_json_pretty(&events, AggregateResolution::RawNs);
    let month_json = aggregate_events_json_pretty(&events, AggregateResolution::SimMonth);
    let week_json  = aggregate_events_json_pretty(&events, AggregateResolution::SimWeek);
    let day_json   = aggregate_events_json_pretty(&events, AggregateResolution::SimDay);

    //----------------------------------------------------------------------
    // 3. Save results
    //----------------------------------------------------------------------
    fs::write("dev/out_raw.json", raw_json).unwrap();
    fs::write("dev/out_month.json", month_json).unwrap();
    fs::write("dev/out_week.json", week_json).unwrap();
    fs::write("dev/out_day.json", day_json).unwrap();

    println!("✔ Aggregated output written to:");
    println!("   dev/out_raw.json");
    println!("   dev/out_month.json");
    println!("   dev/out_week.json");
    println!("   dev/out_day.json");
}
