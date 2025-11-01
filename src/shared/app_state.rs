use std::sync::Arc; // ✅ needed for RwLock
use tokio::sync::RwLock; // ✅ use this one!
use uuid::Uuid;
use crate::supabasic::Supabase;
use crate::sim::runtime::SimulationManager;

#[derive(Clone)]
pub struct AppState {
    pub supa: Supabase,
    pub session_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,
    pub anon_owner_id: Option<Uuid>,
    pub sim_manager: Arc<RwLock<SimulationManager>>, // ✅ now correct type
}

impl AppState {
    /// Build a new AppState from environment
    pub fn new_from_env() -> anyhow::Result<Self> {
        let supa = Supabase::new_from_env()?;
        Ok(Self {
            supa: supa.clone(),
            session_id: None,
            user_owner_id: None,
            anon_owner_id: None,
            sim_manager: Arc::new(RwLock::new(SimulationManager::new(supa.clone()))),

        })
    }

    /// Convenience method for attaching a session or user
    pub fn with_session(mut self, session_id: Uuid, anon_owner_id: Option<Uuid>) -> Self {
        self.session_id = Some(session_id);
        self.anon_owner_id = anon_owner_id;
        self
    }

    /// Alternate constructor if you already have a Supabase instance
    pub fn new(supa: Supabase) -> Self {
        Self {
            supa: supa.clone(),
            session_id: None,
            user_owner_id: None,
            anon_owner_id: None,
            sim_manager: Arc::new(RwLock::new(SimulationManager::new(supa.clone()))),
        }
    }
}
