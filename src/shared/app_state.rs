use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::collections::HashMap;

use crate::core::UserId;
use crate::core::id::WorldId;
use crate::core::{Observer, ObserverId};

use crate::engine::world::state::WorldState;

use crate::core::objex::store::ObjexStore;
use crate::core::objex::geospec::store::GeoSpecStore;
use crate::core::world::world_frame::WorldFrame;
use crate::core::world::world_env_descriptor::WorldSpace;

use crate::shared::world_sources::WorldSource;
use crate::shared::world_sources::json::JsonWorldSource;
use crate::shared::world_sources::SupabaseWorldSource;
use crate::shared::users::user_source::UserSource;
use crate::shared::users::anon_user_source::AnonUserSource;
use crate::shared::session::session_source::SessionSource;
use crate::shared::ownership::ownership_source::OwnershipSource;
use crate::shared::properties::property_source::PropertySource;
use crate::shared::location::location_source::LocationSource;
use crate::shared::location::address_source::AddressSource;
use crate::supabasic::Supabase;
use crate::engine::property_engine::PropertyEngine;

#[derive(Clone)]
pub struct AppState {
    // ---- World loading (persistence boundary) ----
    pub world_source: Arc<dyn WorldSource + Send + Sync>,
    pub user_source: Arc<dyn UserSource + Send + Sync>,
    pub anon_user_source: Arc<dyn AnonUserSource + Send + Sync>,
    pub session_source: Arc<dyn SessionSource + Send + Sync>,
    pub ownership_source: Arc<dyn OwnershipSource + Send + Sync>,
    pub property_source: Arc<dyn PropertySource + Send + Sync>, 
    pub location_source: Arc<dyn LocationSource + Send + Sync>,
    pub address_source: Arc<dyn AddressSource + Send + Sync>,
    pub property_engine: Arc<PropertyEngine>,


    // ---- In-memory world states ----
    pub worlds: Arc<RwLock<HashMap<WorldId, Arc<WorldState>>>>,

    // ---- Immutable world metadata ----
    pub world_frames: Arc<HashMap<WorldId, WorldFrame>>,
    pub world_spaces: Arc<HashMap<WorldId, WorldSpace>>,

    // ---- Shared stores ----
    pub geospec_store: Arc<RwLock<GeoSpecStore>>,
    pub objex_store: Arc<RwLock<ObjexStore>>,

    // ---- Observers / viewers ----
    pub observers: Arc<RwLock<HashMap<ObserverId, Observer>>>,
}


impl AppState {
    pub fn new_from_env() -> anyhow::Result<Self> {
    let supa = Supabase::new_from_env()?;

    let world_source: Arc<dyn WorldSource> =
        if std::env::var("WORLD_SOURCE").as_deref() == Ok("json") {
            Arc::new(JsonWorldSource::from_dir("data/worlds")?)
        } else {
            Arc::new(SupabaseWorldSource { supa })
        };

    let user_source =
        Arc::new(crate::infra::users::supabase_user_source::SupabaseUserSource::new_from_env()?);

    let anon_user_source =
        Arc::new(crate::infra::users::supabase_anon_user_source::SupabaseAnonUserSource::new_from_env()?);

    let session_source =
        Arc::new(crate::infra::session::supabase_session_source::SupabaseSessionSource::new_from_env()?);

    let ownership_source =
        Arc::new(crate::infra::ownership::supabase_ownership_source::SupabaseOwnershipSource::new_from_env()?);

    let property_source =
        Arc::new(crate::infra::properties::supabase_property_source::SupabasePropertySource::new_from_env()?);

    let location_source =
        Arc::new(crate::infra::location::supabase_location_source::SupabaseLocationSource::new_from_env()?);

    let address_source =
        Arc::new(crate::infra::location::supabase_address_source::SupabaseAddressSource::new_from_env()?);

    let property_engine = Arc::new(PropertyEngine {
        property_source: property_source.clone(),
        location_source: location_source.clone(),
        ownership_source: ownership_source.clone(),
    });

    Ok(Self {
        world_source,
        user_source,
        anon_user_source,
        session_source,
        ownership_source,
        property_source,
        location_source,
        address_source,
        property_engine,

        worlds: Arc::new(RwLock::new(HashMap::new())),
        world_frames: Arc::new(HashMap::new()),
        world_spaces: Arc::new(HashMap::new()),

        geospec_store: Arc::new(RwLock::new(GeoSpecStore::new())),
        objex_store: Arc::new(RwLock::new(ObjexStore::new())),

        observers: Arc::new(RwLock::new(HashMap::new())),
    })
}
    
}
