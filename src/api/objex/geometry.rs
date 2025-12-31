use axum::{routing::get, Json, Router};
use crate::core::objex::geospec::api::geometry_templates;
use crate::core::objex::geospec::api::GeometryTemplate;
use crate::shared::app_state::AppState;    

pub fn geometry_routes() -> Router<AppState> {
    Router::new()
        .route("/templates", get(list_geometry_templates))
}

async fn list_geometry_templates() -> Json<Vec<GeometryTemplate>> {
    Json(geometry_templates())
}
