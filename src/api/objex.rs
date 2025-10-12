use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::objex::Objex;
use crate::supabasic::objex::ObjectRecord;
use crate::shared::app_state::AppState;

// ------------------------------------------------------------
// POST /api/objex
// Creates a new Objex entity and its corresponding spawn event.
// ------------------------------------------------------------
pub async fn create_objex(State(app): State<AppState>, Json(obj): Json<Objex>) -> impl IntoResponse {
    let obj_clone = obj.clone(); // for logging

    match ObjectRecord::create(&app.supa, &ObjectRecord::from(obj)).await {
        Ok(rec) => Json(json!({ "entity_id": rec.entity_id, "status": "spawned" })).into_response(),
        Err(e) => {
            eprintln!("Error inserting Objex {:?}: {:?}", obj_clone.name, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Insert failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// GET /api/objex/:entity_id
// Fetch an Objex entity and its material/shape info.
// ------------------------------------------------------------
pub async fn get_objex(State(app): State<AppState>, Path(entity_id): Path<Uuid>) -> impl IntoResponse {
    match ObjectRecord::get(&app.supa, entity_id).await {
        Ok(obj) => Json::<ObjectRecord>(obj).into_response(),

        Err(e) => {
            eprintln!("Error fetching Objex {}: {:?}", entity_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Fetch failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// GET /api/objex
// List all Objex entities
// ------------------------------------------------------------
pub async fn list_objex(State(app): State<AppState>) -> impl IntoResponse {
    match ObjectRecord::list(&app.supa).await {
        Ok(objs) => Json(json!(objs)).into_response(),
        Err(e) => {
            eprintln!("Error listing Objex entities: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("List failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// GET /api/objex/world/:frame_id
// List all Objex entities belonging to a given world frame
// ------------------------------------------------------------
pub async fn list_objex_for_world(State(app): State<AppState>, Path(frame_id): Path<i64>) -> impl IntoResponse {
    let result = app
        .supa
        .from("objex_entities")
        .select("entity_id, name, shape, material_name, material_kind, frame_id")
        .eq("frame_id", &frame_id.to_string())
        .execute()
        .await;

    match result {
        Ok(raw) => Json(raw).into_response(),
        Err(e) => {
            eprintln!("Error listing Objex for frame {}: {:?}", frame_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("List failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ----------------------------------------------------------
// GEt /api/objex/property/:property_id
// List all Objex entities belonging to a given property 
//-----------------------------------------------------------

pub async fn list_objex_for_property(State(app): State<AppState>, Path(property_id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("objex_entities")
        .select("entity_id, name, shape, material_name, material_kind, frame_id, property_id")
        .eq("property_id", &property_id.to_string())
        .execute()
        .await;

    match result {
        Ok(raw) => Json(raw).into_response(),
        Err(e) => {
            eprintln!("Error listing Objex for property {}: {:?}", property_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("List failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// PUT /api/objex/:entity_id
// Replace the entire Objex record
// ------------------------------------------------------------
pub async fn update_objex(
    State(app): State<AppState>,
    Path(entity_id): Path<Uuid>,
    Json(updated): Json<ObjectRecord>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("objex_entities")
        .update(json!(updated))
        .eq("entity_id", &entity_id.to_string())
        .select("*")
        .execute()
        .await;

    match result {
        Ok(res) => Json(json!({ "status": "updated", "result": res })).into_response(),
        Err(e) => {
            eprintln!("Error updating Objex {}: {:?}", entity_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Update failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// PATCH /api/objex/:entity_id
// Partial update
// ------------------------------------------------------------
pub async fn patch_objex(
    State(app): State<AppState>,
    Path(entity_id): Path<Uuid>,
    Json(changes): Json<serde_json::Value>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("objex_entities")
        .update(changes)
        .eq("entity_id", &entity_id.to_string())
        .select("*")
        .execute()
        .await;

    match result {
        Ok(res) => Json(json!({ "status": "patched", "result": res })).into_response(),
        Err(e) => {
            eprintln!("Error patching Objex {}: {:?}", entity_id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": format!("Patch failed: {e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// DELETE /api/objex/:entity_id
// Remove an Objex entity
// ------------------------------------------------------------
pub async fn delete_objex(State(app): State<AppState>, Path(entity_id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("objex_entities")
        .delete()
        .eq("entity_id", &entity_id.to_string())
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "entity_id": entity_id })).into_response(),
        Err(e) => {
            eprintln!("Error deleting Objex {}: {:?}", entity_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Delete failed: {e:?}") })),
            )
                .into_response()
        }
    }
}
