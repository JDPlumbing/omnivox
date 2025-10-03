use axum::{extract::Path, response::IntoResponse, Json};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::supabasic::client::Supabase;
use axum::http::StatusCode;
use crate::supabasic::orm::DbModel;
use crate::supabasic::orm::insert;


/// Represents a row in your `public.users` table (FK to auth.users).
/// This one is for later when you wire up real Supabase auth.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String, // keep String for now, since Supabase returns uuid as text
    pub display_name: String,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl DbModel for User {
    fn table() -> &'static str {
        "users"
    }
}
/// Represents a row in your `anon_users` table (sim/testing).
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AnonUser {
    pub id: Uuid,
    pub display_name: String,
    pub role: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Represents a payload for creating a new anon user.
/// This avoids requiring `id`, `created_at`, `updated_at`.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NewAnonUser {
    pub display_name: String,
    pub role: Option<String>,
}

impl DbModel for AnonUser {
    fn table() -> &'static str {
        "anon_users"
    }
}

impl DbModel for NewAnonUser {
    fn table() -> &'static str {
        "anon_users"
    }
}

/// GET /api/users/{id} → real users
pub async fn get_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match supa
        .from("users")
        .select("*")
        .eq("id", &user_id.to_string())
        .single()
        .await
    {
        Ok(val) => {
            let user: User = serde_json::from_value(val).unwrap();
            Json(user).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching user {}: {:?}", user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

/// GET /api/anon_users/{id}
pub async fn get_anon_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();

    match supa
        .from("anon_users")
        .select("*")
        .eq("id", &user_id.to_string())
        .single()
        .await
    {
        Ok(val) => {
            let user: AnonUser = serde_json::from_value(val).unwrap();
            Json(user).into_response()
        }
        Err(e) => {
            eprintln!("Error fetching anon_user {}: {:?}", user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }
}

/// POST /api/anon_users
pub async fn create_anon_user(Json(payload): Json<NewAnonUser>) -> impl IntoResponse {
    match insert::<NewAnonUser, AnonUser>(&payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            eprintln!("Error creating anon_user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response()
        }
    }

}
