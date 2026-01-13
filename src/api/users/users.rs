use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use serde_json::json;

use crate::supabasic::users::{User, AnonUser, NewAnonUser};
use crate::supabasic::orm::{fetch, list};
use crate::shared::app_state::AppState;
use serde::{Serialize, Deserialize};

// ------------------------------------------------------------
// GET /api/users/:id
// Real Supabase users (auth.users table)
// ------------------------------------------------------------
pub async fn get_user(State(app): State<AppState>, Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("users")
        .select("*")
        .eq("id", &user_id.to_string())
        .single_typed::<User>()
        .await;

    match result {
        Ok(user) => Json(user).into_response(),
        Err(e) => {
            eprintln!("❌ Error fetching user {}: {:?}", user_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "user not found", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// GET /api/users/anon/{id}
// ------------------------------------------------------------
pub async fn get_anon_user(State(_app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match fetch::<AnonUser>(id).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "anon user not found" })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------------
// GET /api/users/anon
// ------------------------------------------------------------
pub async fn list_anon_users(State(_app): State<AppState>) -> impl IntoResponse {
    match list::<AnonUser>().await {
        Ok(users) => Json(users).into_response(),
        Err(e) => {
            eprintln!("❌ Error listing anon users: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "error listing anon users", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// POST /api/users/anon
// ------------------------------------------------------------
pub async fn create_anon_user(
    State(app): State<AppState>,
    Json(payload): Json<NewAnonUser>,
) -> impl IntoResponse {
    let result = app
        .supa
        .from("anon_users")
        .insert(serde_json::to_value(&payload).unwrap())
        .select("*")
        .execute_typed::<AnonUser>()
        .await;

    eprintln!("🧠 create_anon_user result: {:?}", result);

    match result {
        Ok(rows) => Json(json!({ "status": "ok", "inserted": rows })).into_response(),
        Err(e) => {
            eprintln!("❌ Error creating anon_user: {:?}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "insert failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

// ------------------------------------------------------------
// DELETE /api/users/anon/{id}
// ------------------------------------------------------------
pub async fn delete_anon_user(State(app): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = app
        .supa
        .from("anon_users")
        .eq("id", &id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": id })).into_response(),
        Err(e) => {
            eprintln!("❌ Error deleting anon_user {}: {:?}", id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Delete failed", "details": format!("{e:?}") })),
            )
                .into_response()
        }
    }
}

//----------------------------------------------------
//GET /api/users/
//--------------------------------------------------
pub async fn list_users(State(app): State<AppState>) -> impl IntoResponse {
    let result = app
        .supa
        .from("admin_users")
        .select("*")
        .execute();

    match result.await {
        Ok(val) => Json(val).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("{e:?}") })),
        )
        .into_response(),
    }
}

