use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

use crate::{
    supabasic::Supabase,
    supabasic::events::EventRow,
    sim::simulations::simulation::Simulation,
    sim::world::state::{WorldState, World},
};

use crate::core::id::{WorldId, SimulationId, UserId};
use crate::core::id::EntityId;
use crate::core::id::UvoxRegionId;
use crate::core::uvoxid::UvoxId;
pub type SharedManager = Arc<RwLock<SimulationManager>>;

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
    /// Start a new ad-hoc simulation
    /// -----------------------------------------------------------------------
    pub async fn start(&mut self) -> anyhow::Result<SimulationId> {

        //
        // A: Create a VALID SimulationId (Option A — structured, meaningful)
        //
        let sim_id = SimulationId::new(
            WorldId(0),
            UvoxRegionId::new(UvoxId::new(0,0,0), UvoxId::new(100,100,100)),
            crate::core::tdt::sim_time::SimTime::from_ns(0),
            UserId(0),
            0,
        );

        tracing::info!("Starting ad-hoc simulation {:?}", sim_id);

        //
        // B: Since Simulation::new takes a WorldRecord, build a placeholder one
        //
        let world_record = crate::supabasic::worlds::WorldRecord {
            world_id: WorldId(0),
            name: Some("Test-Earth".into()),
            description: None,
            world_epoch: Some(0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let sim = Simulation::new(world_record);

        //
        // Insert into active simulations
        //
        self.simulations.insert(sim_id, sim);
        Ok(sim_id)
    }

    /// -----------------------------------------------------------------------
    /// Perform a simulation tick
    /// -----------------------------------------------------------------------
    pub async fn tick(
        &mut self,
        sim_id: SimulationId,
    ) -> anyhow::Result<Vec<crate::core::chronovox::ChronoEvent>> {

        let sim = self.simulations.get_mut(&sim_id)
            .ok_or_else(|| anyhow::anyhow!("Simulation {:?} not found", sim_id))?;

        // run the tick
        let events = sim.tick();

        //
        // Persist ChronoEvents → EventRow
        //
        for ev in &events {

            // Use entity id from event, or fallback to first entity, or nil
           // remove unwrap_or_else entirely
            let entity_id = ev.entity_id;


            let row = EventRow {
                id: None,
                simulation_id: sim.simulation_id.clone(),   // stored as structured ID in DB
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
    /// List active simulations
    /// -----------------------------------------------------------------------
    pub async fn list(&self) -> anyhow::Result<Vec<SimulationId>> {
        Ok(self.simulations.keys().cloned().collect())
    }

    /// -----------------------------------------------------------------------
    /// Load a simulation state from Supabase into memory
    /// -----------------------------------------------------------------------
    pub async fn load_from_supabase(&mut self, sim_id: SimulationId) -> anyhow::Result<()> {

        tracing::info!("📡 Loading simulation {:?}", sim_id);

        let sim = Simulation::load_from_supabase(&self.supa, sim_id).await?;
        self.simulations.insert(sim_id, sim);

        Ok(())
    }
}
