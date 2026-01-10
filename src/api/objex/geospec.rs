use axum::{
    routing::{post, get},
    extract::{State, Path},
    Json, Router,
    http::StatusCode,

};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::shared::app_state::AppState;
use crate::core::objex::geospec::{AuthoringShape, GeoSpec};

/// ---------------------------------------------------------------------------
/// Routes
/// ---------------------------------------------------------------------------

pub fn geospec_routes() -> Router<AppState> {
    Router::new()
        .route("/compile", post(compile_geospec))
        .route("/{id}", get(get_geospec))
        .route("/", get(list_geospecs))
}

/// ---------------------------------------------------------------------------
/// Records
/// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoSpecRecord {
    pub id: Uuid,
    pub spec: GeoSpec,
}

/// ---------------------------------------------------------------------------
/// Handlers
/// ---------------------------------------------------------------------------

/// Compile AuthoringShape → GeoSpec, assign ID, store
async fn compile_geospec(
    State(state): State<AppState>,
    Json(authoring): Json<AuthoringShape>,
) -> Json<GeoSpecRecord> {
    let spec = authoring.compile();
    let id = Uuid::new_v4();

    state
        .geospec_store
        .write()
        .await
        .insert(id, spec.clone());

    Json(GeoSpecRecord { id, spec })
}

/// Resolve GeoSpec by ID
async fn get_geospec(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GeoSpecRecord>, StatusCode> {
    let store = state.geospec_store.read().await;

    match store.get(&id) {
        Some(spec) => Ok(Json(GeoSpecRecord {
            id,
            spec: spec.clone(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// List all GeoSpecs (editor / debug)
async fn list_geospecs(
    State(state): State<AppState>,
) -> Json<Vec<GeoSpecRecord>> {
    eprintln!("🔥 list_geospecs HIT");
    let store = state.geospec_store.read().await;

    let records = store
        .iter()
        .map(|(id, spec)| GeoSpecRecord {
            id: *id,
            spec: spec.clone(),
        })
        .collect::<Vec<_>>();

    Json(records)
}

