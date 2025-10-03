use uuid::Uuid;
use chrono::{Utc, DateTime};
use serde::Deserialize;
use serde_json::Value;

use crate::supabasic::Supabase;
use crate::tdt::core::TimeDelta;
use crate::chronovox::ChronoEvent;
use crate::sim::world::World;
use crate::sim::simulation::Simulation;
use crate::sim::systems::System;
use crate::sim::error::{OmnivoxError, Result};

#[derive(Deserialize, Debug)]
struct SimulationRow {
    simulation_id: String,
    frame_id: Value,
    tick_rate: Value,
    last_saved: Option<String>,
    owner_id: String,
    metadata: Value,
}

impl Simulation {
    pub async fn load_from_supabase(
        sup: &Supabase,
        sim_id: Uuid,
    ) -> Result<Self> {
        // 1. Fetch simulation metadata
        let raw: Value = sup
            .from("simulations")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        let row_val = raw
            .as_array()
            .and_then(|arr| arr.first().cloned())
            .ok_or_else(|| OmnivoxError::InvalidRow("no rows returned".into()))?;

        let row: SimulationRow = serde_json::from_value(row_val)?;

        let simulation_id = Uuid::parse_str(&row.simulation_id)?;
        let owner_id = Uuid::parse_str(&row.owner_id)?;
        let frame_id = row.frame_id.as_i64()
            .ok_or_else(|| OmnivoxError::InvalidRow("frame_id not an integer".into()))?;
        let tick_rate_val = row.tick_rate.as_i64()
            .ok_or_else(|| OmnivoxError::InvalidRow("tick_rate not an integer".into()))?;
        let _tick_rate = TimeDelta::from_ticks(tick_rate_val, "nanoseconds");

        let last_saved = row.last_saved
            .map(|s| DateTime::parse_from_rfc3339(&s))
            .transpose()
            .map_err(|e| OmnivoxError::InvalidRow(format!("bad timestamp: {e}")))?
            .map(|dt| dt.with_timezone(&Utc));

        // 2. Fetch timeline events
        let mut timeline: Vec<ChronoEvent> = Vec::new();
        let raw_events: Value = sup
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

        // sort by event time
        timeline.sort_by_key(|e| e.t.ticks("nanoseconds"));

        // 3. Build world (minimal data for now)
        let world = World {
            frame_id,
            name: None,
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        // 4. For now, no systems are auto-loaded
        let systems: Vec<Box<dyn System + Send>> = vec![];

        // 5. Build the simulation
        let sim = Simulation {
            simulation_id,
            current_tick: 0,
            frame_id,
            world,
            timeline,
            systems,
        };

        Ok(sim)
    }
}
