use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use supabasic::Supabase;

use chronovox::Timeline;
use tdt::core::TimeDelta;

use crate::sim::world::SimWorld;
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

impl SimWorld {
    pub async fn load_from_supabase(sup: &Supabase, sim_id: Uuid) -> Result<Self> {
        // 1. Fetch as raw JSON (Supabase returns an array of rows)
        let raw: Value = sup
            .from("simulations")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        println!("RAW SIMULATION ROW: {raw:#}");

        // 2. Extract the first row
        let row_val = raw.as_array()
            .and_then(|arr| arr.get(0).cloned())
            .ok_or_else(|| OmnivoxError::InvalidRow("no rows returned".into()))?;

        // 3. Deserialize into SimulationRow
        let row: SimulationRow = serde_json::from_value(row_val)?;

        // 4. Parse UUIDs
        let simulation_id = Uuid::parse_str(&row.simulation_id)?;
        let owner_id = Uuid::parse_str(&row.owner_id)?;

        // 5. Parse numbers safely
        let frame_id = row.frame_id.as_i64()
            .ok_or_else(|| OmnivoxError::InvalidRow("frame_id not an integer".into()))? as u64;

        let tick_rate_val = row.tick_rate.as_i64()
            .ok_or_else(|| OmnivoxError::InvalidRow("tick_rate not an integer".into()))?;

        // 6. Parse timestamp
        let last_saved = row.last_saved
            .map(|s| DateTime::parse_from_rfc3339(&s))
            .transpose()
            .map_err(|e| OmnivoxError::InvalidRow(format!("bad timestamp: {e}")))?
            .map(|dt| dt.with_timezone(&Utc));

        Ok(SimWorld {
            simulation_id,
            frame_id,
            tick_rate: TimeDelta::from_ticks(tick_rate_val, "nanoseconds"),
            last_saved,
            owner_id,
            objects: HashMap::new(),
            timeline: Timeline::new(),
        })
    }
}
