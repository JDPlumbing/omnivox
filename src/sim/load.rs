use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::supabasic::Supabase;
use crate::tdt::core::TimeDelta;
use crate::chronovox::ChronoEvent;
use crate::sim::world::World;
use crate::sim::simulation::Simulation;
use crate::sim::systems::System;
use crate::sim::error::{OmnivoxError, Result};
use crate::supabasic::simulations::SimulationRow;
use crate::supabasic::properties::PropertyRecord;
use crate::supabasic::objex::ObjectRecord;
use crate::supabasic::events::EventRow;
use crate::sim::systems::MovementSystem;



/// Helper: load all data needed to build a runtime `World` from a property.
async fn load_world_from_property(
    sup: &Supabase,
    frame_id: i64,
) -> Result<World> {
    // 1. Fetch the property row (via whatever ID or query you use)
    let property_row: PropertyRecord = sup
        .get_property_by_frame(frame_id)
        .await
        .map_err(|e| OmnivoxError::LoadError(format!("property fetch failed: {:?}", e)))?;

    // 2. Fetch all objex (objects) for this property
    let objexes: Vec<ObjectRecord> = sup
        .list_objex_for_property(property_row.property_id.expect("property_id missing in PropertyRecord"))

        .await
        .unwrap_or_default();

    // 3. Fetch events (if relevant to this world)
    let events: Vec<EventRow> = sup
        .list_events_for_property(property_row.property_id.expect("property_id missing in PropertyRecord"))

        .await
        .unwrap_or_default();

    // 4. Build the runtime world
    Ok(World {
        frame_id,
        name: property_row.name.clone(),

        description: None,

        created_at: property_row.created_at.unwrap_or_else(Utc::now),

        updated_at: Utc::now(),
        deleted_at: None,
        events: events.into_iter().map(|ev| ev.into()).collect(), // convert to ChronoEvent if needed
    })
}

/// Load a complete `Simulation` from Supabase (simulation row + events + hydrated world).
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

        // 2. Fetch timeline events
        let mut timeline: Vec<ChronoEvent> = Vec::new();
        let raw_events = sup
            .from("events")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        if let Some(events) = raw_events.as_array() {
            for ev_val in events {
                let ev: ChronoEvent = serde_json::from_value(ev_val.clone())?;
                timeline.push(ev);
            }
        }

        timeline.sort_by_key(|e| e.t.ticks("nanoseconds"));

        // 3. Hydrate the actual world from Supabase
        let world = load_world_from_property(sup, row.frame_id).await?;

        // 4. Empty system list (you’ll expand this later)
        let systems: Vec<Box<dyn System + Send>> = vec![
            Box::new(MovementSystem),
            // Later: Box::new(CorrosionSystem),
            // Box::new(ErosionSystem),
        ];


        // 5. Build and return full Simulation
        Ok(Simulation {
            simulation_id: row.simulation_id,
            current_tick: 0,
            frame_id: row.frame_id,
            world,
            timeline,
            systems,
        })
    }
}
