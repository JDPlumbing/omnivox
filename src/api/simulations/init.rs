use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::{Utc, TimeZone};

use crate::shared::app_state::AppState;
use crate::supabasic::objex::ObjectRecord;
use crate::tdt::core::TimeDelta;

#[derive(Debug, Deserialize)]
pub struct SimulationInitRequest {
    pub property_id: Uuid,
    pub frame_id: Option<i64>,
    pub tick_rate: Option<i64>,
    pub anon_owner_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct SimulationInitResponse {
    pub status: String,
    pub simulation_id: Uuid,
    pub frame_id: i64,
    pub object_count: usize,
    pub installed_event_count: usize,
}

pub async fn init_simulation(
    State(app): State<AppState>,
    Json(req): Json<SimulationInitRequest>,
) -> impl IntoResponse {
    // 1️⃣ Create the simulation record
    let sim_id = Uuid::new_v4();
    let frame_id = req.frame_id.unwrap_or(0);

    let (user_owner_id, anon_owner_id) = match (req.user_owner_id, req.anon_owner_id) {
        (Some(uid), _) => (json!(uid), Value::Null),
        (None, Some(aid)) => (Value::Null, json!(aid)),
        (None, None) => (Value::Null, json!(Uuid::new_v4())),
    };

    let insert_payload = json!({
        "simulation_id": sim_id,
        "frame_id": frame_id,
        "tick_rate": req.tick_rate.unwrap_or(1),
        "last_saved": Value::Null,
        "metadata": json!({}),
        "user_owner_id": user_owner_id,
        "anon_owner_id": anon_owner_id
    });

    println!(
        "🧩 FINAL JSON TO SUPABASE:\n{}",
        serde_json::to_string_pretty(&insert_payload).unwrap()
    );

    if let Err(e) = app
        .supa
        .from("simulations")
        .insert(insert_payload)
        .select("simulation_id")
        .execute()
        .await
    {
        eprintln!("❌ Simulation insert failed: {:?}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Simulation insert failed: {e:?}") })),
        )
            .into_response();
    }

    // 2️⃣ Fetch property info
    let property: Value = match app
        .supa
        .from("properties")
        .select("year_built")
        .eq("property_id", &req.property_id.to_string())
        .single()
        .await
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("❌ Failed to fetch property info: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to fetch property info: {e:?}") })),
            )
                .into_response();
        }
    };

    let year_built = property["year_built"].as_i64().unwrap_or(2000);
    let install_time = Utc.with_ymd_and_hms(year_built as i32, 1, 1, 0, 0, 0).unwrap();

    let lived_delta = TimeDelta::until_now(install_time);
    let lived_days = lived_delta.ticks("days");
    let lived_pretty = lived_delta.pretty(2);

    println!(
        "🏗️ Property built in {year_built} → lived for {} days ({})",
        lived_days, lived_pretty
    );

    // 3️⃣ Fetch all existing objects for this property
    let objs: Vec<ObjectRecord> = match app
        .supa
        .from("objex_entities")
        .select("entity_id, name, shape, material_name, material_kind, frame_id, property_id")
        .eq("property_id", &req.property_id.to_string())
        .eq("frame_id", &frame_id.to_string())
        .execute_typed()
        .await

    {
        Ok(list) => list,
        Err(e) => {
            eprintln!("❌ Failed to fetch objects: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Fetch objects failed: {e:?}") })),
            )
                .into_response();
        }
    };

    if objs.is_empty() {
        eprintln!("⚠️ No objects found for frame_id {}", frame_id);
    }

    // 4️⃣ Build and normalize Installed events
use std::collections::BTreeMap;

let events: Vec<Value> = objs.iter().map(|o| {
    let mut map = BTreeMap::new();
    map.insert("frame_id".to_string(), json!(o.frame_id));

    map.insert("r_um".to_string(), json!(0));
    map.insert("lat_code".to_string(), json!(0));
    map.insert("lon_code".to_string(), json!(0));
    map.insert("ticks".to_string(), json!(0));
    map.insert("timestamp".to_string(), json!(install_time));
    map.insert("kind".to_string(), json!("Installed"));
    map.insert("payload".to_string(), json!({
        "installed_at_year": year_built,
        "elapsed_since_install_days": lived_days,
        "elapsed_pretty": lived_pretty,
        "source": "property_init"
    }));
    map.insert("entity_id".to_string(), json!(o.entity_id));
    map.insert("simulation_id".to_string(), json!(sim_id));

    Value::Object(map.into_iter().collect())
}).collect();

println!(
    "🧾 Final deterministic insert payload:\n{}",
    serde_json::to_string_pretty(&events[0]).unwrap()
);

// use insert_raw for bulk events!!
let insert_events = app
    .supa
    .from("events")
    .insert_raw(json!(events))
    .select("id")
    .execute()
    .await;




    let inserted_count = match insert_events {
        Ok(v) => v.as_array().map(|a| a.len()).unwrap_or(0),
        Err(e) => {
            eprintln!("⚠️ Failed to insert Installed events: {:?}", e);
            0
        }
    };

    // ✅ Done
    Json(SimulationInitResponse {
        status: "initialized".to_string(),
        simulation_id: sim_id,
        frame_id,
        object_count: objs.len(),
        installed_event_count: inserted_count,
    })
    .into_response()
}
