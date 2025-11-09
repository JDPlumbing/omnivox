use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::supabasic::Supabase;
use crate::tdt::core::TimeDelta;
use crate::chronovox::ChronoEvent;
use crate::sim::world::WorldState;
use crate::supabasic::worlds::WorldRow;

use crate::sim::simulation::Simulation;
use crate::sim::systems::System;
use crate::sim::error::{OmnivoxError, Result};
use crate::supabasic::simulations::SimulationRow;
use crate::supabasic::properties::PropertyRecord;
use crate::supabasic::objex::ObjectRecord;
use crate::supabasic::events::EventRow;
use crate::sim::systems::MovementSystem;
use std::convert::TryFrom;
use crate::objex::core::types::Objex;



/// Helper: load all data needed to build a runtime `WorldState` from a property.
async fn load_world_from_property(
    sup: &Supabase,
    frame_id: i64,
) -> Result<WorldState> {
    // 1. Fetch the property row
    let property_row: PropertyRecord = sup
        .get_property_by_frame(frame_id)
        .await
        .map_err(|e| OmnivoxError::LoadError(format!("property fetch failed: {:?}", e)))?;

    // 2. Fetch all objex for this property
    let objexes: Vec<ObjectRecord> = sup
        .list_objex_for_property(property_row.property_id.expect("property_id missing"))
        .await
        .unwrap_or_default();

    // 3. Fetch events for this property
    let events: Vec<EventRow> = sup
        .list_events_for_property(property_row.property_id.expect("property_id missing"))
        .await
        .unwrap_or_default();

    // 4. Create the metadata world row
    let meta = WorldRow {
        frame_id,
        name: property_row.name.clone(),
        description: None,
        created_at: property_row.created_at.unwrap_or_else(Utc::now),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    // 5. Build runtime world state
    let mut world_state = WorldState::new(meta);
    world_state.events = events;
    world_state.objects = objexes
        .into_iter()
        .filter_map(|r| {
            Objex::try_from(r).ok().map(|o| (o.entity_id.to_string(), o))
        })
        .collect();

    Ok(world_state)
}

/// Load a complete `Simulation` from Supabase (metadata + timeline + hydrated world)
impl Simulation {
    pub async fn load_from_supabase(
        sup: &Supabase,
        sim_id: Uuid,
    ) -> Result<Self> {
        // 1. Fetch simulation metadata
        let row = SimulationRow::get(sup, sim_id)
            .await
            .map_err(|e| OmnivoxError::LoadError(format!("simulation fetch failed: {:?}", e)))?;

        let _tick_rate = TimeDelta::from_ticks(row.tick_rate, "nanoseconds");

        // 2. Load events (timeline)
        let mut timeline: Vec<ChronoEvent> = Vec::new();
        let raw_events = sup
            .from("events")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        // Instead of deserializing straight to ChronoEvent
        if let Some(events) = raw_events.as_array() {
            // Deserialize into EventRow first
            let event_rows: Vec<EventRow> = events
                .iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect();

            // Convert each EventRow → ChronoEvent
            timeline = event_rows.into_iter().map(ChronoEvent::from).collect();
        }

        timeline.sort_by_key(|e| e.t.ticks("nanoseconds"));


        // 3. Hydrate world state
        let world_state = load_world_from_property(sup, row.frame_id).await?;

        // 4. Systems
        let systems: Vec<Box<dyn System + Send + Sync>> = vec![
            Box::new(MovementSystem),
        ];

        use chrono::{Utc, Duration};
        use crate::sim::clock::SimClock;

        let now = Utc::now();
        let start = now - Duration::days(365 * 10); // or derive this from property metadata
        let step = Duration::days(30);
        let clock = SimClock::new(start, now, step);

        // 5. Build Simulation
        Ok(Simulation {
            simulation_id: row.simulation_id,
            current_tick: 0,
            frame_id: row.frame_id,
            world: world_state,
            timeline,
            systems,
            clock,
        })
    }
}



