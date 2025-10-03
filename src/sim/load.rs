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

impl Simulation {
    pub async fn load_from_supabase(
        sup: &Supabase,
        sim_id: Uuid,
    ) -> Result<Self> {
        // 1. Fetch simulation row
        let row = SimulationRow::get(sup, sim_id).await?;

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

        // 3. Build placeholder world (later: fetch real one)
        let world = World {
            frame_id: row.frame_id,
            name: None,
            description: None,
            created_at: row.last_saved.unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        // 4. Empty systems for now
        let systems: Vec<Box<dyn System + Send>> = vec![];

        // 5. Return assembled Simulation
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
