use axum::{
    extract::{Path, Json},
    response::IntoResponse,
    http::StatusCode,
};
use uuid::Uuid;

use crate::supabasic::users::{User, AnonUser, NewAnonUser}; // bring in both structs
use crate::supabasic::client::Supabase;
use crate::supabasic::orm::{fetch, list, insert};


/// GET /api/users/:id (real Supabase users — tied to auth.users)
pub async fn get_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match supa
        .from("users")
        .select("*")
        .eq("id", &user_id.to_string())
        .execute_one::<User>()
        .await
    {
        Ok(user) => Json(user).into_response(),
        Err(e) => {
            eprintln!("Error fetching user {}: {:?}", user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error fetching user").into_response()
        }
    }
}

/// GET /api/anon_users/{id}
pub async fn get_anon_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    match fetch::<AnonUser>(id).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

/// GET /api/anon_users
pub async fn list_anon_users() -> impl IntoResponse {
    match list::<AnonUser>().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response(),
    }
}

/// POST /api/anon_users

pub async fn create_anon_user(Json(payload): Json<NewAnonUser>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    let result = supa
        .from("anon_users")
        .insert(serde_json::json!([&payload]))   // 👈 wrap in array
        .select("*")
        .execute()
        .await;

    eprintln!("DEBUG raw insert response: {:?}", result);

    match result {
        Ok(val) => Json(val).into_response(),
        Err(e) => {
            eprintln!("Error creating anon_user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("insert failed: {:?}", e)).into_response()
        }
    }
}
