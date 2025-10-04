// src/api/objex.rs
use axum::{extract::{Path, Json}, response::IntoResponse};
use axum::http::StatusCode;
use uuid::Uuid;
use chrono::Utc;

use crate::supabasic::Supabase;
use crate::supabasic::objex::ObjectRecord;
use crate::supabasic::events::EventRow;
use crate::objex::Objex;

#[derive(serde::Serialize)]
pub struct ObjexDto {
    pub entity_id: Uuid,
    pub name: String,
    pub material_name: String,
    pub material_kind: String,
}

impl From<ObjectRecord> for ObjexDto {
    fn from(r: ObjectRecord) -> Self {
        ObjexDto {
            entity_id: r.entity_id.expect("missing entity_id"),
            name: r.name,
            material_name: r.material_name,
            material_kind: r.material_kind,
        }
    }
}

/// GET /api/objex
pub async fn list_objex() -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match ObjectRecord::list(&supa).await {
        Ok(rows) => {
            let dto: Vec<ObjexDto> = rows.into_iter().map(ObjexDto::from).collect();
            Json(dto).into_response()
        }
        Err(e) => {
            eprintln!("Error listing objex: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

/// GET /api/objex/:id
pub async fn get_objex(Path(id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    match ObjectRecord::get(&supa, id).await {
        Ok(obj) => Json(ObjexDto::from(obj)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

/// POST /api/simulations/:sim_id/objex
pub async fn create_objex_handler(
    Path(sim_id): Path<Uuid>,
    Json(payload): Json<ObjectRecord>,
) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match ObjectRecord::create(&supa, &payload).await {
        Ok(objex) => {
            // create spawn event for this object
            let spawn_event = EventRow {
                id: Uuid::new_v4(),
                simulation_id: sim_id,
                entity_id: objex.entity_id.expect("entity_id missing after insert"),
                frame_id: 0,
                r_um: 0,
                lat_code: 0,
                lon_code: 0,
                ticks: 0,
                timestamp: Utc::now(),
                kind: "spawn".to_string(),
                move_offset: None,
                payload: None,
                created_at: Utc::now(),
            };

            if let Err(e) = EventRow::create(&supa, &spawn_event).await {
                eprintln!("Error creating spawn event: {:?}", e);
            }

            Json(ObjexDto::from(objex)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error creating objex: {}", e),
        ).into_response(),
    }
}
