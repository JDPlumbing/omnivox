//! Simulation runtime manager (rewritten scaffolding version B)
//!
//! This version is designed to:
//! - Use `SimulationConfig` for starting new simulations
//! - Support loading persisted simulations from Supabase
//! - Provide a `start_dev()` helper for local testing
//! - Cleanly separate concerns: config, runtime creation, persistence, ticking
//!
//! NOTE: All persistence loaders (`SupabaseSimLoader`, `PersistedSimState`) are stubs.
//! You will fill them in once you define how your ECS/world state is saved.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

use crate::supabasic::Supabase;
use crate::supabasic::events::EventRow;
use crate::supabasic::worlds::WorldRecord;

use crate::sim::simulations::simulation::Simulation;
use crate::sim::world::state::{WorldState, World};

use crate::core::chronovox::ChronoEvent;
use crate::core::id::{SimulationId, WorldId, UserId};
use crate::sim::simulations::simulation_config::SimulationConfig;
use crate::core::id::UvoxRegionId;
use crate::core::uvoxid::{UvoxId, RUm, LatCode, LonCode};

pub type SharedManager = Arc<RwLock<SimulationManager>>;

/// ---------------------------------------------------------------------------
/// Persisted Simulation State (stub)
/// ---------------------------------------------------------------------------
/// Eventually this will contain:
/// - serialized ECS component tables
/// - entities
/// - world clock
/// - region/world metadata
///
/// For now it is a placeholder that allows compilation.
#[derive(Debug, Clone)]
pub struct PersistedSimState {
    // TODO: Fill with your actual ECS snapshot format
    pub placeholder: bool,
}

/// ---------------------------------------------------------------------------
/// Stub loader for Supabase → persisted state + config + world record
/// ---------------------------------------------------------------------------
pub struct SupabaseSimLoader;

impl SupabaseSimLoader {
    pub async fn load_everything(
        supa: &Supabase,
        sim_id: SimulationId,
    ) -> anyhow::Result<(WorldRecord, PersistedSimState, SimulationConfig)> {

        // --- Load world record ---
        let world_record = Self::load_world_record(supa, sim_id.world).await?;

        // --- Load config ---
        let cfg = Self::load_config(supa, sim_id).await?;

        // --- Load persisted ECS/world state ---
        let persisted = Self::load_runtime_state(supa, sim_id).await?;

        Ok((world_record, persisted, cfg))
    }

    async fn load_world_record(
        supa: &Supabase,
        world_id: WorldId,
    ) -> anyhow::Result<WorldRecord> {
        // TODO: replace with real Supabase fetch
        Ok(WorldRecord {
            world_id,
            name: Some("Loaded-World".into()),
            description: None,
            world_epoch: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    async fn load_config(
        supa: &Supabase,
        sim_id: SimulationId,
    ) -> anyhow::Result<SimulationConfig> {
        // TODO: Replace with real persisted config fetch
        Ok(SimulationConfig::basic(
            sim_id.world,
            sim_id.region,
            UserId(0),
        ))
    }

    async fn load_runtime_state(
        _supa: &Supabase,
        _sim_id: SimulationId,
    ) -> anyhow::Result<PersistedSimState> {
        // TODO: Load entity/component tables, clocks, etc.
        Ok(PersistedSimState { placeholder: true })
    }
}

/// ---------------------------------------------------------------------------
/// SimulationManager — orchestrates running simulations in memory
/// ---------------------------------------------------------------------------
pub struct SimulationManager {
    pub supa: Supabase,
    pub simulations: HashMap<SimulationId, Simulation>,
}

impl SimulationManager {

    pub fn new(supa: Supabase) -> Self {
        Self {
            supa,
            simulations: HashMap::new(),
        }
    }

    /// -----------------------------------------------------------------------
    /// Start a new simulation from config
    /// -----------------------------------------------------------------------
    pub async fn start(
        &mut self,
        cfg: SimulationConfig,
    ) -> anyhow::Result<SimulationId> {

        let sim_id = SimulationId::new(
            cfg.world_id,
            cfg.region,
            cfg.start_time,
            cfg.user_id,
            cfg.branch,
        );

        tracing::info!("Starting simulation {:?}", sim_id);

        // Load world metadata from DB
        let world_record = SupabaseSimLoader::load_world_record(&self.supa, cfg.world_id).await?;

        // Construct the simulation from config
        let sim = Simulation::new_from_config(&cfg, world_record);

        self.simulations.insert(sim_id, sim);
        Ok(sim_id)
    }

    /// -----------------------------------------------------------------------
    /// Start a dev-mode simulation with hardcoded testing values
    /// -----------------------------------------------------------------------
    pub async fn start_dev(&mut self) -> anyhow::Result<SimulationId> {
        let cfg = SimulationConfig::basic(
            WorldId(0),
            UvoxRegionId::default(),
            UserId(0),
        );

        self.start(cfg).await
    }

    /// -----------------------------------------------------------------------
    /// Load persisted simulation from Supabase and rehydrate runtime
    /// -----------------------------------------------------------------------
    pub async fn load_from_supabase(
        &mut self,
        sim_id: SimulationId,
    ) -> anyhow::Result<()> {

        tracing::info!("📡 Loading simulation {:?}", sim_id);

        let (world_record, persisted, cfg) =
            SupabaseSimLoader::load_everything(&self.supa, sim_id).await?;

        let sim = Simulation::from_persisted(world_record, persisted, cfg);

        self.simulations.insert(sim_id, sim);
        Ok(())
    }

    /// -----------------------------------------------------------------------
    /// Perform a simulation tick
    /// -----------------------------------------------------------------------
    pub async fn tick(
        &mut self,
        sim_id: SimulationId,
    ) -> anyhow::Result<Vec<ChronoEvent>> {

        let sim = self.simulations.get_mut(&sim_id)
            .ok_or_else(|| anyhow::anyhow!("Simulation {:?} not found", sim_id))?;

        let events = sim.tick();

        for ev in &events {
            let row = EventRow {
                id: None,
                simulation_id: sim.simulation_id.clone(),
                entity_id: ev.entity_id,
                world_id: sim.world_id,
                ticks: ev.t.as_ns(),
                timestamp: Some(Utc::now()),
                kind: format!("{:?}", ev.kind),
                payload: ev.payload.clone(),
                created_at: Some(Utc::now()),
            };

            let _ = EventRow::create(&self.supa, &row).await;
        }

        tracing::info!("Ticked sim {:?} → {} events", sim_id, events.len());
        Ok(events)
    }

    /// -----------------------------------------------------------------------
    /// Stop simulation & delete from memory
    /// -----------------------------------------------------------------------
    pub async fn stop(&mut self, sim_id: SimulationId) -> anyhow::Result<bool> {
        Ok(self.simulations.remove(&sim_id).is_some())
    }

    /// -----------------------------------------------------------------------
    /// List active simulations in memory
    /// -----------------------------------------------------------------------
    pub async fn list(&self) -> anyhow::Result<Vec<SimulationId>> {
        Ok(self.simulations.keys().cloned().collect())
    }
}

