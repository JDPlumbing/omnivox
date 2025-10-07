use axum::{
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use serde_json::json;

use crate::supabasic::users::{User, AnonUser, NewAnonUser};
use crate::supabasic::client::Supabase;
use crate::supabasic::orm::{fetch, list};

/// GET /api/users/:id (real Supabase users — tied to auth.users)
pub async fn get_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("users")
        .select("*")
        .eq("id", &user_id.to_string())
        .single_typed::<User>()
        .await;

    match result {
        Ok(user) => Json(user).into_response(),
        Err(e) => {
            eprintln!("❌ Error fetching user {}: {:?}", user_id, e);
            (StatusCode::NOT_FOUND, "user not found").into_response()
        }
    }
}

/// GET /api/users/anon/{id}
pub async fn get_anon_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    match fetch::<AnonUser>(id).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "anon user not found").into_response(),
    }
}

/// GET /api/users/anon
pub async fn list_anon_users() -> impl IntoResponse {
    match list::<AnonUser>().await {
        Ok(users) => Json(users).into_response(),
        Err(e) => {
            eprintln!("❌ Error listing anon users: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error listing anon users").into_response()
        }
    }
}

/// POST /api/users/anon
pub async fn create_anon_user(Json(payload): Json<NewAnonUser>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("anon_users")
        .insert(serde_json::to_value(&payload).unwrap()) // ✅ no array
        .select("*")
        .execute_typed::<AnonUser>()
        .await;

    eprintln!("🧠 create_anon_user result: {:?}", result);

    match result {
        Ok(rows) => Json(json!({ "status": "ok", "inserted": rows })).into_response(),
        Err(e) => {
            eprintln!("❌ Error creating anon_user: {:?}", e);
            (StatusCode::BAD_REQUEST, format!("insert failed: {:?}", e)).into_response()
        }
    }
}

/// DELETE /api/users/anon/{id}
pub async fn delete_anon_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("anon_users")
        .eq("id", &id.to_string())
        .delete()
        .execute()
        .await;

    match result {
        Ok(_) => Json(json!({ "status": "deleted", "id": id })).into_response(),
        Err(e) => {
            eprintln!("❌ Error deleting anon_user {}: {:?}", id, e);
            (StatusCode::BAD_REQUEST, format!("Delete failed: {e:?}")).into_response()
        }
    }
}
