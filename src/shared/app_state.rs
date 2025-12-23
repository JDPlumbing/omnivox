use std::sync::Arc; 
use std::collections::HashMap;
use tokio::sync::RwLock; 
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::engine::simulations::runtime::SimulationManager;
use crate::core::UserId;
use crate::shared::viewer_state::ViewerRegistry;

#[derive(Clone)]
pub struct AppState {
    pub supa: Supabase,
    pub session_id: Option<Uuid>,
    pub user_owner_id: Option<UserId>,
    pub anon_owner_id: Option<UserId>,

    pub sim_manager: Arc<RwLock<SimulationManager>>,
    pub viewer_registry: Arc<RwLock<ViewerRegistry>>,


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
            viewer_registry: Arc::new(RwLock::new(ViewerRegistry::default())),


            sim_manager: Arc::new(RwLock::new(SimulationManager::new())), // ✔ FIXED
        })
    }

    /// Attach session
    pub fn with_session(mut self, session_id: Uuid, anon_owner_id: Option<UserId>) -> Self {
        self.session_id = Some(session_id);
        self.anon_owner_id = anon_owner_id;
        self
    }

    /// Alternate constructor
    pub fn new(supa: Supabase) -> Self {
        Self {
            supa: supa.clone(),
            session_id: None,
            user_owner_id: None,
            anon_owner_id: None,
            viewer_registry: Arc::new(RwLock::new(ViewerRegistry::default())),

            sim_manager: Arc::new(RwLock::new(SimulationManager::new())), // ✔ FIXED
        }
    }
}
