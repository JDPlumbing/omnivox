use uuid::Uuid;
use chrono::Utc;

use supabasic::Supabase;
use objex::{Objex, insert_objex, MaterialLink, Shape};
use chronovox::{ChronoEvent, EventKind, UvoxId, insert_event_for_entity};
use tdt::core::TimeDelta;

use crate::sim::error::OmnivoxError;

/// Persist just the Objex record
pub async fn persist_objex(
    sup: &Supabase,
    obj: &Objex,
) -> Result<Uuid, OmnivoxError> {
    let id = obj.entity_id;
    insert_objex(sup, id, obj).await?;
    Ok(id)
}

/// Persist a Chronovox event tied to an entity
pub async fn persist_event(
    sup: &Supabase,
    entity_id: Uuid,
    event: &ChronoEvent,
) -> Result<Uuid, OmnivoxError> {
    let ev_id = insert_event_for_entity(sup, entity_id, event).await?;
    Ok(ev_id)
}

/// Create an entity, persist its Objex, and spawn event in one shot.
/// Returns the `entity_id` and `event_id`.
pub async fn spawn_entity_with_objex(
    sup: &Supabase,
    simulation_id: Uuid,
    frame_id: i64,
    objex: Objex,
    uvox: UvoxId,
) -> Result<(Uuid, Uuid), OmnivoxError> {
    let entity_id = objex.entity_id;

    // 1. Persist the Objex
    insert_objex(sup, entity_id, &objex).await?;

    // 2. Create a spawn event
    let event = ChronoEvent {
        id: uvox.clone(),
        t: TimeDelta::from_ticks(0, "nanoseconds"), // default spawn at tick 0
        kind: EventKind::Spawn,
        payload: Some(serde_json::json!({
            "simulation_id": simulation_id,
            "frame_id": frame_id,
            "observed": true,
            "historical_inferred": false
        })),
    };

    // 3. Persist the event tied to this entity
    let event_id = insert_event_for_entity(sup, entity_id, &event).await?;

    Ok((entity_id, event_id))
}
