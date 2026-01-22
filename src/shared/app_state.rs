use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::supabasic::Supabase;
//use crate::engine::simulations::runtime::SimulationManager;
//use crate::shared::viewer_state::ViewerRegistry;
use crate::core::UserId;
use crate::core::objex::store::ObjexStore;
use crate::core::objex::geospec::store::GeoSpecStore;
use std::collections::HashMap;
use crate::core::world::world_frame::WorldFrame;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::id::WorldId;
use crate::core::{Observer, ObserverId};


#[derive(Clone)]
pub struct AppState {
    pub supa: Supabase,

    pub session_id: Option<Uuid>,
    pub supabase_user_id: Option<Uuid>,
    pub user_owner_id: Option<UserId>,
    pub anon_owner_id: Option<UserId>,

    //pub sim_manager: Arc<RwLock<SimulationManager>>,
    //pub viewer_registry: Arc<RwLock<ViewerRegistry>>,
    pub geospec_store: Arc<RwLock<GeoSpecStore>>,

    /// TEMPORARY in-memory Objex template store
    pub objex_store: Arc<RwLock<ObjexStore>>,

    pub world_frames: HashMap<WorldId, WorldFrame>,
    pub world_spaces: HashMap<WorldId, WorldSpace>,
    pub observers: Arc<RwLock<HashMap<ObserverId, Observer>>>
}

impl AppState {
    pub fn new_from_env() -> anyhow::Result<Self> {
        let supa = Supabase::new_from_env()?;

        Ok(Self {
            supa,

            session_id: None,
            supabase_user_id: None,

            user_owner_id: None,
            anon_owner_id: None,

            //sim_manager: Arc::new(RwLock::new(SimulationManager::new())),
            //viewer_registry: Arc::new(RwLock::new(ViewerRegistry::default())),
            geospec_store: Arc::new(RwLock::new(GeoSpecStore::new())),
            objex_store: Arc::new(RwLock::new(ObjexStore::new())),
            world_frames: HashMap::new(),
            world_spaces: HashMap::new(),
            observers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn new(supa: Supabase) -> Self {
        Self {
            supa,

            session_id: None,
            supabase_user_id: None,

            user_owner_id: None,
            anon_owner_id: None,

            //sim_manager: Arc::new(RwLock::new(SimulationManager::new())),
            //viewer_registry: Arc::new(RwLock::new(ViewerRegistry::default())),
            geospec_store: Arc::new(RwLock::new(GeoSpecStore::new())),
            objex_store: Arc::new(RwLock::new(ObjexStore::new())),
            world_frames: HashMap::new(),
            world_spaces: HashMap::new(),
            observers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_session(
        mut self,
        session_id: Uuid,
        anon_owner_id: Option<UserId>,
    ) -> Self {
        self.session_id = Some(session_id);
        self.anon_owner_id = anon_owner_id;
        self
    }
}
