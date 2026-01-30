// src/core/tdt/sim_aggregate.rs
//
// Deterministic time-bucket aggregation for ChronoEvents.
// Works with the fixed simulation calendar and the extended formats
// defined in sim_display.rs.


use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_display::{TimeFormat, format_simtime};
use crate::core::tdt::sim_calendar::*;

use serde_json::json;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ChronoEvent {
    pub entity_id: u64,
    pub world_id: u64,
    pub kind: String,
    pub payload: serde_json::Value,
    pub t: SimTime,
}
/// ---------------------------------------------------------------------------
/// Resolution modes (bucket sizes)
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub enum AggregateResolution {
    RawNs,       // never aggregate, full fidelity
    SimMonth,    // bucket by SimMonth index
    SimWeek,     // bucket by SimWeek index
    SimDay,      // bucket by SimDay index
    SimHour,     // bucket by SimHour index
}

/// Compute which bucket this timestamp belongs to.
fn bucket_index(t: SimTime, res: AggregateResolution) -> i64 {
    match res {
        AggregateResolution::RawNs => t.as_ns() as i64,
        AggregateResolution::SimMonth => (t.as_ns() / NANOS_PER_MONTH) as i64,
        AggregateResolution::SimWeek  => (t.as_ns() / NANOS_PER_WEEK) as i64,
        AggregateResolution::SimDay   => (t.as_ns() / NANOS_PER_DAY) as i64,
        AggregateResolution::SimHour  => (t.as_ns() / NANOS_PER_HOUR) as i64,
    }
}

/// Choose the appropriate display format when producing JSON output.
fn bucket_display_format(res: AggregateResolution) -> TimeFormat {
    match res {
        AggregateResolution::RawNs   => TimeFormat::RawNs,
        AggregateResolution::SimMonth => TimeFormat::SimMonth,
        AggregateResolution::SimWeek  => TimeFormat::SimWeek,
        AggregateResolution::SimDay   => TimeFormat::SimDay,
        AggregateResolution::SimHour  => TimeFormat::Rfc3339, // hour-level readable
    }
}

/// ---------------------------------------------------------------------------
/// Aggregate a list of ChronoEvents into deterministic buckets.
/// Returns: Vec<json!({...})>
/// ---------------------------------------------------------------------------
pub fn aggregate_events(
    events: &[ChronoEvent],
    resolution: AggregateResolution,
) -> Vec<serde_json::Value> {

    if events.is_empty() {
        return vec![];
    }

    // 1. Bucket map: bucket_index → Vec<ChronoEvent>
    let mut buckets: BTreeMap<i64, Vec<ChronoEvent>> = BTreeMap::new();

    for e in events.iter().cloned() {
        let key = bucket_index(e.t, resolution);
        buckets.entry(key).or_default().push(e);
    }

    let fmt = bucket_display_format(resolution);

    // 2. Convert into aggregate JSON blocks
    let mut out = Vec::new();
    for (bucket_id, evs) in buckets.iter() {
        // bucket timestamp reused from the first event in the bucket
        let display_time = if let Some(e0) = evs.first() {
            format_simtime(e0.t, fmt)
        } else {
            "<empty>".to_string()
        };

        let events_json: Vec<_> = evs
            .iter()
            .map(|e| json!({
                "entity_id": e.entity_id,
                "world_id":  e.world_id,
                "kind":      e.kind,
                "payload":   e.payload,
                "t_raw":     e.t.as_ns(),
                "t_display": format_simtime(e.t, fmt),
            }))
            .collect();

        out.push(json!({
            "bucket": bucket_id,
            "time": display_time,
            "count": events_json.len(),
            "events": events_json
        }));
    }

    out
}

/// ---------------------------------------------------------------------------
/// Convenience export: full JSON array as a pretty string
/// ---------------------------------------------------------------------------
pub fn aggregate_events_json_pretty(
    events: &[ChronoEvent],
    resolution: AggregateResolution,
) -> String {
    let blocks = aggregate_events(events, resolution);
    serde_json::to_string_pretty(&blocks).unwrap()
}

/// ---------------------------------------------------------------------------
/// Convenience export: compact JSON
/// ---------------------------------------------------------------------------
pub fn aggregate_events_json(
    events: &[ChronoEvent],
    resolution: AggregateResolution,
) -> String {
    let blocks = aggregate_events(events, resolution);
    serde_json::to_string(&blocks).unwrap()
}
