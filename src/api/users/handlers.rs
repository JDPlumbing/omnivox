use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;


use uuid::Uuid;
use reqwest::Client;
use std::env;
use crate::shared::app_state::AppState;
use crate::shared::{ AuthContext, AccountRole};
use super::create::{CreateUserPayload, create_user_service};
use axum::extract::Extension;

//--------------------------
// CREATE USER
//------------------------
pub async fn create_user(
    State(app): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    match create_user_service(&app, payload).await {
        Ok(user) => Json(user).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

//--------------------------
// DELETE USER
//------------------------
pub async fn delete_user(
    State(_app): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let supabase_url =
        env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let service_role_key =
        env::var("SUPABASE_SERVICE_ROLE_KEY").expect("SUPABASE_SERVICE_ROLE_KEY not set");

    let client = Client::new();

    let res = client
        .delete(format!(
            "{}/auth/v1/admin/users/{}",
            supabase_url, user_id
        ))
        .header("Authorization", format!("Bearer {}", service_role_key))
        .header("apikey", service_role_key)
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => {
            Json(json!({ "status": "deleted", "id": user_id })).into_response()
        }
        Ok(r) => {
            let text = r.text().await.unwrap_or_default();
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": text })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
            .into_response(),
    }
}
//--------------------------
// GET ME
//--------------------------
pub async fn get_me(
    Extension(auth): Extension<AuthContext>,
    State(app): State<AppState>,
) -> impl IntoResponse {

    // 1️⃣ Identity (resolved by middleware)
    let supa_user_id = auth.supabase_user_id;
    let user_id = auth.user_id;

    // 2️⃣ Query property relationship
    let result = app
        .supa
        .from("user_properties")
        .select(
            "role,property:properties(property_id,name,region:uvox_regions(min_r_um,min_lat_code,min_lon_code,max_r_um,max_lat_code,max_lon_code),world:worlds(world_id,name))"
        )
        .eq("user_id", &supa_user_id.to_string())
        .maybe_single_typed::<serde_json::Value>()
        .await;

    let row = match result {
        Ok(Some(r)) => r,
        Ok(None) => {
            // user exists, but has no property assignment
            return Json(json!({
                "user": {
                    "id": user_id.to_string(),
                    "account_role": "user"
                },
                "property": null,
                "property_role": null,
                "world": null
            }))
            .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    // 3️⃣ Extract fields
    let property_role = row["role"].as_str().unwrap_or("viewer");
    let property = &row["property"];
    let region = &property["region"];
    let world = &property["world"];

    // 4️⃣ TEMP: account role logic
    // (later this moves to a real table)
    let account_role = match auth.account_role {
        AccountRole::Root => "root",
        AccountRole::User => "user",
    };


    // 5️⃣ Final contract response
    Json(json!({
        "user": {
            "id": user_id.to_string(),
            "account_role": account_role
        },
        "property_role": property_role,
        "property": {
            "id": property["property_id"],
            "name": property["name"],
            "region": {
                "min": {
                    "r_um": region["min_r_um"],
                    "lat_code": region["min_lat_code"],
                    "lon_code": region["min_lon_code"]
                },
                "max": {
                    "r_um": region["max_r_um"],
                    "lat_code": region["max_lat_code"],
                    "lon_code": region["max_lon_code"]
                }
            }
        },
        "world": {
            "id": world["world_id"],
            "name": world["name"]
        }
    }))
    .into_response()
}
