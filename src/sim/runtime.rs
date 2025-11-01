use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    sim::simulation::Simulation,
    sim::world::World,
    sim::systems::System,
    supabasic::Supabase,
};

/// Thread-safe handle for managing multiple live simulations
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

    /// 🚀 Start a brand-new ad-hoc simulation (no DB lookups)
    pub async fn start(&mut self) -> anyhow::Result<Uuid> {
        let sim_id = Uuid::new_v4();
        tracing::info!("Starting ad-hoc simulation {sim_id}...");

        let simulation = Simulation {
            simulation_id: sim_id,
            current_tick: 0,
            frame_id: 0, // placeholder world ID — can link later
            world: World::default(),
            timeline: vec![],
            systems: vec![], // preload systems later
        };

        self.simulations.insert(sim_id, simulation);
        Ok(sim_id)
    }

    /// Advance one tick in a running simulation
    pub async fn tick(&mut self, sim_id: Uuid) -> anyhow::Result<Vec<crate::chronovox::ChronoEvent>> {
        if let Some(sim) = self.simulations.get_mut(&sim_id) {
            Ok(sim.tick())
        } else {
            Err(anyhow::anyhow!("Simulation not found: {sim_id}"))
        }
    }

    /// Stop (remove) a running simulation
    pub async fn stop(&mut self, sim_id: Uuid) -> anyhow::Result<bool> {
        Ok(self.simulations.remove(&sim_id).is_some())
    }

    /// List all active simulation IDs
    pub async fn list(&self) -> anyhow::Result<Vec<Uuid>> {
        Ok(self.simulations.keys().cloned().collect())
    }
}
