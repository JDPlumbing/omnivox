use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    supabasic::{Supabase},
    supabasic::events::EventRow,
    sim::simulations::simulation::Simulation,
    sim::world::state::{World, WorldState},
};

pub type SharedManager = Arc<RwLock<SimulationManager>>;

pub struct SimulationManager {
    pub supa: Supabase,
    pub simulations: HashMap<Uuid, Simulation>,
}

impl SimulationManager {
    pub fn new(supa: Supabase) -> Self {
        Self {
            supa,
            simulations: HashMap::new(),
        }
    }

    /// Start a brand-new ad-hoc runtime simulation.
    /// Uses an empty default world record (world_id = 0, name = Test-Earth).
    pub async fn start(&mut self) -> anyhow::Result<Uuid> {
        let sim_id = Uuid::new_v4();

        tracing::info!("Starting ad-hoc simulation {sim_id}...");

        // Simulation::new expects a WorldRecord, not a runtime World.
        // So we convert runtime World::default() → WorldRecord.
        let world_record = crate::supabasic::worlds::WorldRecord {
            world_id: 0,
            name: Some("Test-Earth".into()),
            description: None,
            world_epoch: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let sim = Simulation::new(world_record);

        self.simulations.insert(sim_id, sim);
        Ok(sim_id)
    }


    /// Perform one simulation tick.
    pub async fn tick(
        &mut self,
        sim_id: Uuid
    ) -> anyhow::Result<Vec<crate::core::chronovox::ChronoEvent>> {

        let sim = self.simulations.get_mut(&sim_id)
            .ok_or_else(|| anyhow::anyhow!("Simulation {sim_id} not found"))?;

        let events = sim.tick();

        //
        // For each ChronoEvent → write EventRow to DB
        //
        for ev in &events {

            // entity_id extraction
            let entity_id = ev.payload
                .as_ref()
                .and_then(|p| p.get("entity_id"))
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .or_else(|| sim.world.entities.keys().next().cloned())
                .unwrap_or_else(|| Uuid::nil());

            let row = EventRow {
                id: None,
                simulation_id: sim.simulation_id,
                entity_id,
                world_id: sim.world_id,
                ticks: ev.t.as_ns(),
                timestamp: Some(Utc::now()),
                kind: format!("{:?}", ev.kind),
                payload: ev.payload.clone(),
                created_at: Some(Utc::now()),
            };

            let _ = EventRow::create(&self.supa, &row).await;
        }

        tracing::info!("Ticked sim {sim_id} → {} events", events.len());

        Ok(events)
    }


    pub async fn stop(&mut self, sim_id: Uuid) -> anyhow::Result<bool> {
        Ok(self.simulations.remove(&sim_id).is_some())
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Uuid>> {
        Ok(self.simulations.keys().cloned().collect())
    }

    pub async fn load_from_supabase(&mut self, sim_id: Uuid) -> anyhow::Result<()> {
        tracing::info!("📡 Loading simulation {} from Supabase", sim_id);

        let sim = Simulation::load_from_supabase(&self.supa, sim_id).await?;
        self.simulations.insert(sim_id, sim);
        Ok(())
    }
}
