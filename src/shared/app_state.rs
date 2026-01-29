use std::sync::Arc;
use crate::shared::world_sources::catalog::source::WorldCatalog;
use crate::shared::world_sources::state::source::WorldStateSource;
use crate::shared::identity::auth_source::AuthSource;
use crate::shared::identity::identity_source::IdentitySource;
use crate::shared::users::user_source::UserSource;
use crate::shared::users::anon_user_source::AnonUserSource;
use crate::shared::session::session_source::SessionSource;
use crate::shared::properties::property_source::PropertySource;
use crate::shared::location::location_source::LocationSource;
use crate::shared::location::address_source::AddressSource;
use crate::engine::user::user_engine::UserEngine;
//use crate::engine::world::world_engine::WorldEngine;
use crate::engine::location::location_engine::LocationEngine;
use crate::engine::property::property_engine::PropertyEngine;
use crate::engine::time::time_engine::TimeEngine;

#[derive(Clone)]
pub struct AppState {
    pub time_engine: Arc<TimeEngine>,
    // ---- World persistence ----
    pub world_catalog: Arc<dyn WorldCatalog + Send + Sync>,
    pub world_state_source: Arc<dyn WorldStateSource + Send + Sync>,

    // ---- Identity / auth ----
    pub auth_source: Arc<dyn AuthSource + Send + Sync>,
    pub identity_source: Arc<dyn IdentitySource + Send + Sync>,
    pub user_source: Arc<dyn UserSource + Send + Sync>,
    pub anon_user_source: Arc<dyn AnonUserSource + Send + Sync>,
    pub session_source: Arc<dyn SessionSource + Send + Sync>,

    // ---- Other domain sources ----
    pub property_source: Arc<dyn PropertySource + Send + Sync>,
    pub location_source: Arc<dyn LocationSource + Send + Sync>,
    pub address_source: Arc<dyn AddressSource + Send + Sync>,

    // ---- Engines ----
    pub user_engine: Arc<UserEngine>,
    //pub world_engine: Arc<WorldEngine>,
    pub location_engine: Arc<LocationEngine>,
    pub property_engine: Arc<PropertyEngine>,
}



impl AppState {
    pub fn new(
        time_engine: Arc<TimeEngine>,
        world_catalog: Arc<dyn WorldCatalog + Send + Sync>,
        world_state_source: Arc<dyn WorldStateSource + Send + Sync>,

        auth_source: Arc<dyn AuthSource + Send + Sync>,
        identity_source: Arc<dyn IdentitySource + Send + Sync>,
        user_source: Arc<dyn UserSource + Send + Sync>,
        anon_user_source: Arc<dyn AnonUserSource + Send + Sync>,
        session_source: Arc<dyn SessionSource + Send + Sync>,

        property_source: Arc<dyn PropertySource + Send + Sync>,
        location_source: Arc<dyn LocationSource + Send + Sync>,
        address_source: Arc<dyn AddressSource + Send + Sync>,

        user_engine: Arc<UserEngine>,
        //world_engine: Arc<WorldEngine>,
        location_engine: Arc<LocationEngine>,
        property_engine: Arc<PropertyEngine>,
    ) -> Self {
        Self {
            time_engine,
            world_catalog,
            world_state_source,

            auth_source,
            identity_source,
            user_source,
            anon_user_source,
            session_source,

    
            property_source,
            location_source,
            address_source,

            user_engine,
            //world_engine,
            location_engine,
            property_engine,
        }
    }
    pub fn from_env() -> anyhow::Result<Self> {
        crate::app::bootstrap::build_app_state_from_env()
    }
}
