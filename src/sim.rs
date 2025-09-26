use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use objex::Objex;
use chronovox::{Timeline, UvoxId};
use tdt::core::TimeDelta;

use crate::error::{OmnivoxError, Result}; // 👈 new error type

pub struct SimWorld {
    /// Simulation container ID (primary key in `simulations` table)
    pub simulation_id: Uuid,

    /// Frame of reference for UvoxId coordinates (0 = canonical Earth, etc.)
    pub frame_id: u64,

    /// How much time one tick represents (ns per tick or similar)
    pub tick_rate: TimeDelta,

    /// Last time this simulation was saved
    pub last_saved: Option<DateTime<Utc>>,

    /// Owner of this simulation (foreign key to `auth.users` table)
    pub owner_id: Uuid,

    /// The entities that exist in this world, keyed by their spatial UvoxId
    pub objects: HashMap<UvoxId, Objex>,

    /// Chronological record of events in this simulation
    pub timeline: Timeline,
}

impl Default for SimWorld {
    fn default() -> Self {
        Self {
            simulation_id: Uuid::parse_str("b691967d-8820-4f81-ab32-a9e7a10189f7")
                .expect("hardcoded UUID should parse"),
            frame_id: 0, // test world always 0
            tick_rate: TimeDelta::from_ticks(1, "nanoseconds"),
            last_saved: None,
            owner_id: Uuid::parse_str("4ea96b3f-51d7-4238-bd18-2f7fd8be26ec")
                .expect("hardcoded UUID should parse"),
            objects: HashMap::new(),
            timeline: Timeline::new(),
        }
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
struct SimulationRow {
    simulation_id: String,          // comes back as a string
    frame_id: serde_json::Value,    // flexible: may be number or wrapped map
    tick_rate: serde_json::Value,   // same
    last_saved: Option<String>,     // ISO string timestamp
    owner_id: String,               // string UUID
    metadata: serde_json::Value,    // raw json blob
}


use supabasic::Supabase;
// no more `supabasic::Result` import

use chronovox::fetch_events_for_entity;
use serde_json::Value;

impl SimWorld {
    pub async fn load_from_supabase(
        sup: &Supabase,
        sim_id: Uuid,
    ) -> Result<Self> {
        // 1. Grab raw JSON from Supabase
        let raw: Value = sup
            .from("simulations")
            .select("*")
            .eq("simulation_id", &sim_id.to_string())
            .execute()
            .await?;

        println!("RAW Supabase response: {:#}", raw);

        // 2. Extract first row if it's wrapped in an array
        let row = raw
            .get(0)
            .ok_or_else(|| OmnivoxError::InvalidRow("no rows returned".into()))?;

        // 3. Pull fields manually
        let sim_id_str = row
            .get("simulation_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OmnivoxError::InvalidRow("missing simulation_id".into()))?;
        let simulation_id = Uuid::parse_str(sim_id_str)?;

        let frame_id = row
            .get("frame_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| OmnivoxError::InvalidRow("frame_id not an integer".into()))? as u64;

        let tick_rate = row
            .get("tick_rate")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| OmnivoxError::InvalidRow("tick_rate not an integer".into()))?;

        let owner_id_str = row
            .get("owner_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OmnivoxError::InvalidRow("missing owner_id".into()))?;
        let owner_id = Uuid::parse_str(owner_id_str)?;

        let last_saved = row
            .get("last_saved")
            .and_then(|v| v.as_str())
            .map(|s| s.parse::<DateTime<Utc>>())
            .transpose()?; // Option<Result<T>> → Result<Option<T>>

        // 4. Build SimWorld
        let world = SimWorld {
            simulation_id,
            frame_id,
            tick_rate: TimeDelta::from_ticks(tick_rate, "nanoseconds"),
            last_saved,
            owner_id,
            objects: HashMap::new(),
            timeline: Timeline::new(),
        };

        Ok(world)
    }
}
