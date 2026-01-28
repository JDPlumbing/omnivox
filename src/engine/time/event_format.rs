use crate::core::tdt::sim_display::{format_simtime, TimeFormat};
use crate::core::chronovox::ChronoEvent;
use serde_json::json;
//NOTE: THIS MIGHT NEED TO GO IN CHORNOVOX LATER
pub fn format_event_block(
    events: &[ChronoEvent],
    fmt: TimeFormat,
) -> serde_json::Value {
    let time_string = events
        .first()
        .map(|e| format_simtime(e.t, fmt))
        .unwrap_or_else(|| "<no-time>".to_string());

    let ev_list: Vec<_> = events
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

    json!({
        "time": time_string,
        "events": ev_list
    })
}
