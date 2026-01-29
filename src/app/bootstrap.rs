use std::sync::{Arc};
use crate::shared::app_state::AppState;
use crate::shared::world_sources::catalog::source::WorldCatalog;
use crate::shared::world_sources::state::source::WorldStateSource;
use crate::infra::world_sources::catalog::json::JsonWorldCatalog;
use crate::infra::inmemory::address::InMemoryAddressSource;
use crate::infra::inmemory::identity::InMemoryIdentitySource;
use crate::infra::inmemory::location::InMemoryLocationSource;

use crate::infra::inmemory::property::InMemoryPropertySource;
use crate::infra::inmemory::session::InMemorySessionSource;
use crate::engine::user::user_engine::UserEngine;
//use crate::engine::world::WorldEngine;
use crate::engine::location::location_engine::LocationEngine;
use crate::engine::property::property_engine::PropertyEngine;
use crate::infra::inmemory::users::InMemoryUserSource;
use crate::infra::inmemory::anon_users::InMemoryAnonUserSource;
use crate::infra::inmemory::auth::InMemoryAuthSource;
use crate::infra::inmemory::world_state::InMemoryWorldStateSource;
//use crate::engine::world::loader::WorldLoader;
use crate::engine::time::time_engine::TimeEngine;
use crate::shared::identity::auth_source::AuthSource;
use crate::shared::identity::identity_source::IdentitySource;
use crate::shared::users::user_source::UserSource;
use crate::shared::users::anon_user_source::AnonUserSource;
use crate::shared::session::session_source::SessionSource;
use crate::infra::dev::auth::DevAuthSource;
use crate::infra::dev::identity::DevIdentitySource;
use crate::shared::identity::auth_context::AccountRole;


pub fn build_app_state_from_env() -> anyhow::Result<AppState> {
    let time_engine = Arc::new(TimeEngine::default());

    // --- World ---
    let world_catalog: Arc<dyn WorldCatalog> =
        Arc::new(JsonWorldCatalog::from_dir("data/worlds")?);

    let world_state_source: Arc<dyn WorldStateSource> =
        Arc::new(InMemoryWorldStateSource::default());


// Define once, up front
let dev_user_id = crate::core::UserId::from_uuid(
    uuid::uuid!("00000000-0000-0000-0000-000000000001")
);

let (
    auth_source,
    identity_source,
    user_source,
    anon_user_source,
    session_source,
): (
    Arc<dyn AuthSource + Send + Sync>,
    Arc<dyn IdentitySource + Send + Sync>,
    Arc<dyn UserSource + Send + Sync>,
    Arc<dyn AnonUserSource + Send + Sync>,
    Arc<dyn SessionSource + Send + Sync>,
) = if cfg!(debug_assertions) {
    // 🔧 DEV MODE
    (
        Arc::new(DevAuthSource::new(dev_user_id)),
        Arc::new(DevIdentitySource::new(dev_user_id, AccountRole::Root)),
        Arc::new(InMemoryUserSource::default()),
        Arc::new(InMemoryAnonUserSource::default()),
        Arc::new(InMemorySessionSource::default()),
    )
} else {
    // 🔐 NORMAL MODE
    (
        Arc::new(InMemoryAuthSource::default()),
        Arc::new(InMemoryIdentitySource::default()),
        Arc::new(InMemoryUserSource::default()),
        Arc::new(InMemoryAnonUserSource::default()),
        Arc::new(InMemorySessionSource::default()),
    )
};



    // --- Other domain sources ---
    let property_source = Arc::new(InMemoryPropertySource::default());
    let location_source = Arc::new(InMemoryLocationSource::default());
    let address_source = Arc::new(InMemoryAddressSource::default());

    // --- Engines ---
    let user_engine = Arc::new(UserEngine::new(
        auth_source.clone(),
        identity_source.clone(),
        user_source.clone(),
        anon_user_source.clone(),
        session_source.clone(),
    ));

    // let loader = Arc::new(WorldLoader::new(
    //     world_catalog.clone(),
    //     world_state_source.clone(),
    // ));

    // let world_engine = Arc::new(WorldEngine::new(loader));

    let location_engine =
        Arc::new(LocationEngine::new(location_source.clone()));

    let property_engine = Arc::new(PropertyEngine::new(
        property_source.clone(),
        location_source.clone(),
    ));

    Ok(AppState::new(
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
    ))
}
