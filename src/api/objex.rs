// src/api/objex.rs
use axum::{extract::Path, response::IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::objex::Objex;
use crate::objex::persist::{insert_objex, fetch_objex};
use crate::supabasic::objex::ObjectRecord;

/// POST /api/objex
/// Creates a new Objex entity and its corresponding spawn event.
pub async fn create_objex(Json(obj): Json<Objex>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    let obj_clone = obj.clone(); // 👈 keep one copy for logging

    match ObjectRecord::create(&supa, &ObjectRecord::from(obj)).await {
        Ok(rec) => Json(json!({ "entity_id": rec.entity_id, "status": "spawned" })).into_response(),
        Err(e) => {
            eprintln!("Error inserting Objex {:?}: {:?}", obj_clone.name, e);
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



// -------------------------
// LIST ALL
// -------------------------

/// GET /api/objex
/// List all Objex entities in the database.
pub async fn list_objex() -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    match ObjectRecord::list(&supa).await {
        Ok(objs) => Json(json!(objs)).into_response(),
        Err(e) => {
            eprintln!("Error listing Objex entities: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("List failed: {e:?}")).into_response()
        }
    }
}

// -------------------------
// LIST BY WORLD FRAME_ID
// -------------------------

/// GET /api/objex/world/:frame_id
/// List all objects belonging to a given world frame.
pub async fn list_objex_for_world(Path(frame_id): Path<i64>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Supabase init error: {e}"),
            )
                .into_response();
        }
    };

    {
        let builder = supa
            .from("objex_entities")
            .select("entity_id,name,shape,material_name,material_kind,frame_id")
            .eq("frame_id", &frame_id.to_string());

        let result = builder.execute().await;

        match result {
            Ok(raw) => Json(raw).into_response(),
            Err(e) => {
                eprintln!("Error listing Objex for frame {}: {:?}", frame_id, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("List failed: {e:?}"),
                )
                    .into_response()
            }
        }
    }
}

// -------------------------
// UPDATE FULL (PUT)
// -------------------------

/// PUT /api/objex/:entity_id
/// Replace the entire Objex record (name, shape, material).
pub async fn update_objex(Path(entity_id): Path<Uuid>, Json(updated): Json<ObjectRecord>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    let result = supa
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
            (StatusCode::BAD_REQUEST, format!("Update failed: {e:?}")).into_response()
        }
    }
}

// -------------------------
// UPDATE PARTIAL (PATCH)
// -------------------------

/// PATCH /api/objex/:entity_id
/// Update only selected fields (partial update).
pub async fn patch_objex(Path(entity_id): Path<Uuid>, Json(changes): Json<serde_json::Value>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    let result = supa
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
            (StatusCode::BAD_REQUEST, format!("Patch failed: {e:?}")).into_response()
        }
    }
}
// -------------------------
// DELETE
// -------------------------

/// DELETE /api/objex/:entity_id
/// Remove an Objex entity (and ideally its related events later).
pub async fn delete_objex(Path(entity_id): Path<Uuid>) -> impl IntoResponse {
    let supa = match Supabase::new_from_env() {
        Ok(client) => client,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Supabase init error: {e}")).into_response(),
    };

    let result = supa
        .from("objex_entities")
        .delete()
        .eq("entity_id", &entity_id.to_string())
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "entity_id": entity_id })).into_response(),
        Err(e) => {
            eprintln!("Error deleting Objex {}: {:?}", entity_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Delete failed: {e:?}")).into_response()
        }
    }
}