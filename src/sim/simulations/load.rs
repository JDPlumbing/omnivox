// Std
use std::convert::TryFrom;

// External crates
use uuid::Uuid;
use chrono::Utc;

// Supabase / DB row types
use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRecord;
use crate::supabasic::entity::EntityRecord;
use crate::supabasic::simulations::SimulationRow;
use crate::supabasic::events::EventRow;

// Core library types
use crate::core::tdt::time_delta::TimeDelta;
use crate::core::chronovox::ChronoEvent;

// Simulation types / systems / errors
use crate::sim::world::{WorldState, World};
use crate::sim::simulations::simulation::Simulation;
use crate::sim::systems::{System, MovementSystem};
use crate::sim::error::{OmnivoxError, Result};
use crate::sim::entities::SimEntity;
use crate::sim::clock::SimClock;

/// ---------------------------------------------------------------------------
///  LOAD A COMPLETE RUNTIME WORLD FROM SUPABASE BY world_id
/// ---------------------------------------------------------------------------
async fn load_world(
    sup: &Supabase,
    world_id: i64,
) -> Result<WorldState> {

    //
    // 1. Load world metadata from DB
    //
    let rec = WorldRecord::get(sup, world_id)
        .await
        .map_err(|e| OmnivoxError::LoadError(format!("world fetch failed: {:?}", e)))?;

    // Convert DB → runtime
    let meta: World = rec.into();

    //
    // 2. Load entities belonging to this world
    //
    let entity_rows: Vec<EntityRecord> =
        EntityRecord::list_for_world(sup, world_id)
            .await
            .unwrap_or_default();

    let mut entities = std::collections::HashMap::new();
    for row in entity_rows {
        match SimEntity::try_from(row) {
            Ok(ent) => {
                entities.insert(ent.entity_id, ent);
            }
            Err(e) => {
                eprintln!("⚠ Failed to convert EntityRecord → SimEntity: {:?}", e);
            }
        }
    }

    //
    // 3. Load world events
    //
    let events: Vec<EventRow> =
        EventRow::list_for_world(sup, world_id)
            .await
            .unwrap_or_default();

    //
    // 4. Build runtime world state
    //
    let mut state = WorldState::new(meta);
    state.entities = entities;
    state.events = events;

    Ok(state)
}

/// ---------------------------------------------------------------------------
///  LOAD A COMPLETE SIMULATION: metadata + timeline + hydrated world
/// ---------------------------------------------------------------------------
impl Simulation {
    pub async fn load_from_supabase(
        sup: &Supabase,
        sim_id: Uuid,
    ) -> Result<Self> {

        //
        // 1. Load SimulationRow metadata
        //
        let row = SimulationRow::get(sup, sim_id)
            .await
            .map_err(|e| OmnivoxError::LoadError(format!("simulation fetch failed: {:?}", e)))?;

        let _tick_rate = TimeDelta::from_ticks(row.tick_rate, "nanoseconds");

        //
        // 2. Load simulation timeline events
        //
        let mut timeline: Vec<ChronoEvent> = Vec::new();

        let raw_events = sup
            .from("events")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        if let Some(arr) = raw_events.as_array() {
            let rows: Vec<EventRow> =
                arr.iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect();

            timeline = rows.into_iter().map(ChronoEvent::from).collect();
        }

        timeline.sort_by_key(|e| e.t.ticks("nanoseconds"));

        //
        // 3. Load world state for the referenced world_id
        //
        let world_state = load_world(sup, row.world_id).await?;

        //
        // 4. Initialize systems
        //
        let systems: Vec<Box<dyn System + Send + Sync>> = vec![
            Box::new(MovementSystem),
        ];

        //
        // 5. Construct clock
        //
        use chrono::{Duration};
        let now = Utc::now();
        let start = now - Duration::days(365 * 10);
        let step = Duration::days(30);
        let clock = SimClock::from_wall_dates(start, now, step);
        let sim_time = clock.current;

        //
        // 6. Build Simulation object
        //
        Ok(Simulation {
            simulation_id: row.simulation_id,
            world_id: row.world_id,
            world: world_state,
            timeline,
            systems,
            sim_time,
            clock,
        })
    }
}
