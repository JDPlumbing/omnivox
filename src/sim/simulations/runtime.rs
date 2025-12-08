use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::core::chronovox::ChronoEvent;
use crate::sim::simulations::simulation::Simulation;
use crate::sim::simulations::simulation_config::SimulationConfig;
use crate::supabasic::worlds::WorldRecord;
use crate::sim::simulations::persist::state::PersistedSimState;

/// Shared manager between API and runtime
pub type SharedManager = Arc<RwLock<SimulationManager>>;

/// =================================================================
/// SimulationManager — Store everything by **String api_id**
/// =================================================================
pub struct SimulationManager {
    pub simulations: HashMap<String, Simulation>, // API ID → Simulation
}

impl SimulationManager {
    pub fn new() -> Self {
        Self {
            simulations: HashMap::new(),
        }
    }

    // ---------------------------------------------------------------
    // Start a new simulation
    // ---------------------------------------------------------------
    pub async fn start(
        &mut self,
        cfg: SimulationConfig,
        world_record: WorldRecord,
    ) -> anyhow::Result<String> 
    {
        let real_id = cfg.to_simulation_id();
        let api_id = real_id.to_api_id();   // <-- ALWAYS store as String

        let sim = Simulation::new_from_config(&cfg, world_record);

        self.simulations.insert(api_id.clone(), sim);

        Ok(api_id)
    }

    // ---------------------------------------------------------------
    // Tick simulation
    // ---------------------------------------------------------------
    pub async fn tick(
        &mut self,
        api_id: String,
    ) -> anyhow::Result<Vec<ChronoEvent>> 
    {
        let sim = self.simulations
            .get_mut(&api_id)
            .ok_or_else(|| anyhow::anyhow!("Simulation not found"))?;

        Ok(sim.tick())
    }

    // ---------------------------------------------------------------
    // Stop simulation
    // ---------------------------------------------------------------
    pub async fn stop(&mut self, api_id: String) -> anyhow::Result<bool> {
        Ok(self.simulations.remove(&api_id).is_some())
    }

    // ---------------------------------------------------------------
    // List active simulations
    // ---------------------------------------------------------------
    pub async fn list(&self) -> anyhow::Result<Vec<String>> {
        Ok(self.simulations.keys().cloned().collect())
    }

    // ---------------------------------------------------------------
    // Build snapshot (store by String key)
    // ---------------------------------------------------------------
    pub async fn snapshot(
        &self,
        api_id: String,
    ) -> anyhow::Result<PersistedSimState> 
    {
        let sim = self.simulations
            .get(&api_id)
            .ok_or_else(|| anyhow::anyhow!("Simulation not found"))?;

        Ok(PersistedSimState::from_runtime(sim))
    }

    // ---------------------------------------------------------------
    // Load snapshot (uses String key as well)
    // ---------------------------------------------------------------
    pub async fn load_snapshot(
        &mut self,
        api_id: String,
        world_record: WorldRecord,
        cfg: SimulationConfig,
        snapshot: PersistedSimState,
    ) -> anyhow::Result<()> 
    {
        let sim = snapshot.to_runtime(world_record, cfg);

        self.simulations.insert(api_id, sim);

        Ok(())
    }
}
