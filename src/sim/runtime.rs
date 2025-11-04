use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::supabasic::events::EventRow;
use chrono::Utc;
use crate::supabasic::WorldRow;

use crate::{
    sim::simulation::Simulation,
    sim::world::WorldState,
    sim::systems::System,
    supabasic::Supabase,
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

    pub async fn start(&mut self) -> anyhow::Result<Uuid> {
        let sim_id = Uuid::new_v4();
        tracing::info!("Starting ad-hoc simulation {sim_id}...");

        // 🧠 Use the constructor that sets up world, systems, and components
        let simulation = Simulation::new(WorldRow::default());

        self.simulations.insert(sim_id, simulation);
        Ok(sim_id)
    }


    pub async fn tick(&mut self, sim_id: Uuid) -> anyhow::Result<Vec<crate::chronovox::ChronoEvent>> {
        if let Some(sim) = self.simulations.get_mut(&sim_id) {
            let events = sim.tick();

            for ev in &events {
                let event_row = EventRow {
                    id: None,
                    simulation_id: sim.simulation_id,
                    entity_id: ev.payload
                        .as_ref()
                        .and_then(|p| p.get("entity_id"))
                        .and_then(|v| v.as_str())
                        .and_then(|s| Uuid::parse_str(s).ok())
                        .unwrap_or_else(|| {
                            sim.world
                                .objects
                                .keys()
                                .next()
                                .and_then(|k| Uuid::parse_str(k).ok())
                                .unwrap_or_default()
                        }),

                    frame_id: sim.frame_id,
                    r_um: ev.id.r_um,
                    lat_code: ev.id.lat_code,
                    lon_code: ev.id.lon_code,
                    ticks: ev.t.ticks("nanoseconds"),
                    timestamp: Some(Utc::now()),
                    kind: format!("{:?}", ev.kind),
                    payload: ev.payload.clone(),
                    created_at: Some(Utc::now()),
                };

                let _ = EventRow::create(&self.supa, &event_row).await;
            }

            tracing::info!("Ticked sim {sim_id} → {} events", events.len());
            Ok(events)
        } else {
            anyhow::bail!("Simulation {sim_id} not found");
        }
    }

    pub async fn stop(&mut self, sim_id: Uuid) -> anyhow::Result<bool> {
        Ok(self.simulations.remove(&sim_id).is_some())
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Uuid>> {
        Ok(self.simulations.keys().cloned().collect())
    }

    pub async fn load_from_supabase(&mut self, sim_id: Uuid) -> anyhow::Result<()> {
        tracing::info!("📡 Loading simulation {} from Supabase", sim_id);

        let sim = crate::sim::simulation::Simulation::load_from_supabase(&self.supa, sim_id).await?;
        self.simulations.insert(sim_id, sim);
        Ok(())
    }
}
