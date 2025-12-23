// Std
use std::convert::TryFrom;

// External crates
use chrono::Utc;

// Supabase / DB row types
use crate::supabasic::Supabase;
use crate::supabasic::worlds::WorldRow;
use crate::supabasic::entity::EntityRow;
use crate::supabasic::simulations::SimulationRow;
use crate::supabasic::events::EventRow;

// Core types
use crate::core::tdt::time_delta::TimeDelta;
use crate::core::chronovox::ChronoEvent;
use crate::core::id::{WorldId, SimulationId};
use crate::core::tdt::sim_time::SimTime;

// Simulation types
use crate::engine::world::{WorldState, World};
use crate::engine::simulations::simulation::Simulation;
use crate::engine::systems::{System, MovementSystem};
use crate::engine::error::{OmnivoxError, Result};
use crate::engine::entities::SimEntity;
use crate::engine::clock::SimClock;


/// ---------------------------------------------------------------------------
/// Load runtime world
/// ---------------------------------------------------------------------------
async fn load_world(
    sup: &Supabase,
    world_id: WorldId,
) -> Result<WorldState> {

    let rec = WorldRow::get(sup, world_id)
        .await
        .map_err(|e| OmnivoxError::LoadError(format!("world fetch failed: {:?}", e)))?;

    let meta: World = rec.into();

    let entity_rows = EntityRow::list_for_world(sup, world_id)
        .await
        .unwrap_or_default();

    let mut entities = std::collections::HashMap::new();
    for row in entity_rows {
        match SimEntity::try_from(row) {
            Ok(ent) => { entities.insert(ent.id, ent); }
            Err(e) => eprintln!("⚠ Failed SimEntity conversion: {:?}", e),
        }
    }

    let mut state = WorldState::new(meta);
    state.entities = entities;

    Ok(state)
}


/// ---------------------------------------------------------------------------
/// Load full simulation: metadata + timeline + world
/// ---------------------------------------------------------------------------
impl Simulation {
    pub async fn load_from_supabase(
        sup: &Supabase,
        sim_id: SimulationId,
    ) -> Result<Self> {

        //
        // 1. Load SimulationRow
        //
        let row = SimulationRow::get(sup, sim_id.clone())
            .await
            .map_err(|e| OmnivoxError::LoadError(format!("simulation fetch failed: {:?}", e)))?;

        let _tick_rate = TimeDelta::from_ticks(row.tick_rate, "nanoseconds");

        //
        // 2. Load timeline events
        //
        let sim_json_id = serde_json::to_value(&row.simulation_id).unwrap();

        let raw_events = sup
            .from("events")
            .select("*")
            .eq("simulation_id", sim_json_id.to_string().as_str())

            .execute()
            .await
            .map_err(|e| OmnivoxError::LoadError(format!("event fetch failed: {:?}", e)))?;

        let mut timeline = Vec::new();

        if let Some(arr) = raw_events.as_array() {
            for val in arr {
                if let Ok(r) = serde_json::from_value::<EventRow>(val.clone()) {
                    timeline.push(ChronoEvent::from(r));
                }
            }
        }

        timeline.sort_by_key(|e| e.t.ticks("nanoseconds"));

        //
        // 3. Load world state
        //
        let world_state = load_world(sup, row.world_id).await?;

        //
        // 4. Install ECS systems
        //
        let systems: Vec<Box<dyn System + Send + Sync>> = vec![
            Box::new(MovementSystem),
        ];

        //
        // 5. Construct simulation clock
        //
        use chrono::Duration;
        let now = Utc::now();
        let start = now - Duration::days(365 * 10);
        let step = Duration::days(30);

        let clock = SimClock::from_wall_dates(start, now, step);
        let sim_time = clock.current;

        //
        // 6. SimulationId is ALREADY stored in row.simulation_id
        //
        let sim_meta = row.simulation_id.clone();

        //
        // 7. Build Simulation struct
        //
        Ok(Simulation {
            simulation_id: sim_meta,
            world_id: row.world_id,
            sim_time,
            clock,
            world: world_state,
            systems,
            timeline,
        })
    }
}
