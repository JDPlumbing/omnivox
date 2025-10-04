// src/api/objex.rs
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::objex::Objex;
use crate::objex::persist::{insert_objex, fetch_objex};

/// POST /api/objex
/// Creates a new Objex entity and its corresponding spawn event.
pub async fn create_objex(Json(obj): Json<Objex>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    match insert_objex(&supa, &obj).await {
        Ok(_) => Json(json!({ "entity_id": obj.entity_id, "status": "spawned" })).into_response(),
        Err(e) => {
            eprintln!("Error inserting Objex {:?}: {:?}", obj.name, e);
            (StatusCode::BAD_REQUEST, format!("Insert failed: {e:?}")).into_response()
        }
    }
}

/// GET /api/objex/:entity_id
/// Fetch an Objex entity and its material/shape info.
pub async fn get_objex(Path(entity_id): Path<Uuid>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    match fetch_objex(&supa, entity_id).await {
        Ok(obj) => Json(obj).into_response(),
        Err(e) => {
            eprintln!("Error fetching Objex {}: {:?}", entity_id, e);
            (StatusCode::NOT_FOUND, format!("Fetch failed: {e:?}")).into_response()
        }
    }
}
