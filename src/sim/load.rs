use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::supabasic::Supabase;

use crate::chronovox::Timeline;
use crate::tdt::core::TimeDelta;
use crate::sim::systems::movement::MovementSystem;

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

use crate::chronovox::ChronoEvent;

impl SimWorld {
    pub async fn load_from_supabase(sup: &Supabase, sim_id: Uuid) -> Result<Self> {
        // 1. Fetch simulation metadata
        let raw: Value = sup
            .from("simulations")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        println!("RAW SIMULATION ROW: {raw:#}");

        let row_val = raw.as_array()
            .and_then(|arr| arr.first().cloned())
            .ok_or_else(|| OmnivoxError::InvalidRow("no rows returned".into()))?;

        let row: SimulationRow = serde_json::from_value(row_val)?;
        let simulation_id = Uuid::parse_str(&row.simulation_id)?;
        let owner_id = Uuid::parse_str(&row.owner_id)?;
        let frame_id = row.frame_id.as_i64()
            .ok_or_else(|| OmnivoxError::InvalidRow("frame_id not an integer".into()))? as u64;
        let tick_rate_val = row.tick_rate.as_i64()
            .ok_or_else(|| OmnivoxError::InvalidRow("tick_rate not an integer".into()))?;
        let last_saved = row.last_saved
            .map(|s| DateTime::parse_from_rfc3339(&s))
            .transpose()
            .map_err(|e| OmnivoxError::InvalidRow(format!("bad timestamp: {e}")))? 
            .map(|dt| dt.with_timezone(&Utc));

        // 2. Build world
        let mut world = SimWorld {
            simulation_id,
            frame_id,
            tick_rate: TimeDelta::from_ticks(tick_rate_val, "nanoseconds"),
            last_saved,
            owner_id,
            objects: HashMap::new(),
            timeline: Timeline::new(),
            current_tick: 0,
            persist_events: true,
        };

        // 3. Fetch and hydrate events
        let raw_events: Value = sup
            .from("events")
            .select("*, order(timestamp.asc)")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        if let Some(events) = raw_events.as_array() {
            for ev_val in events {
                let ev: ChronoEvent = serde_json::from_value(ev_val.clone())?;
                world.timeline.push(ev);
            }
        }

        Ok(world)
    }
}

