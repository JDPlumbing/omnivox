use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::{json};
use crate::supabasic::client::Supabase;
use crate::supabasic::orm::{DbModel, insert};

/// Real user row (from auth.users joined to public.users)
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl DbModel for User {
    fn table() -> &'static str { "users" }
}

/// Simulation / testing anon user
#[derive(Debug, Deserialize, Serialize)]
pub struct AnonUser {
    pub id: Uuid,
    pub display_name: String,
    pub role: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl DbModel for AnonUser {
    fn table() -> &'static str { "anon_users" }
}

/// Payload for creating new anon user
#[derive(Debug, Deserialize, Serialize)]
pub struct NewAnonUser {
    pub display_name: String,
    pub role: Option<String>,
}
impl DbModel for NewAnonUser {
    fn table() -> &'static str { "anon_users" }
}

/// Payload for updating an anon user
#[derive(Debug, Deserialize, Serialize)]
pub struct AnonUserUpdate {
    pub display_name: Option<String>,
    pub role: Option<String>,
}

/// GET /api/users/{id}
pub async fn get_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    let result = supa
        .from("users")
        .select("*")
        .eq("id", &user_id.to_string())
        .single()
        .await;

    match result {
        Ok(val) => {
            let user: User = serde_json::from_value(val).unwrap();
            Json(user).into_response()
        }
        Err(e) => {
            eprintln!("❌ Error fetching user {}: {:?}", user_id, e);
            (StatusCode::NOT_FOUND, "user not found").into_response()
        }
    }
}

/// GET /api/anon_users/{id}
pub async fn get_anon_user(Path(user_id): Path<Uuid>) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    let result = supa
        .from("anon_users")
        .select("*")
        .eq("id", &user_id.to_string())
        .single()
        .await;

    match result {
        Ok(val) => {
            let user: AnonUser = serde_json::from_value(val).unwrap();
            Json(user).into_response()
        }
        Err(e) => {
            eprintln!("❌ Error fetching anon_user {}: {:?}", user_id, e);
            (StatusCode::NOT_FOUND, "anon_user not found").into_response()
        }
    }
}

/// POST /api/anon_users
pub async fn create_anon_user(Json(payload): Json<NewAnonUser>) -> impl IntoResponse {
    match insert::<NewAnonUser, AnonUser>(&payload).await {
        Ok(user) => Json(json!({ "status": "ok", "inserted": user })).into_response(),
        Err(e) => {
            eprintln!("❌ Error creating anon_user: {:?}", e);
            (StatusCode::BAD_REQUEST, format!("insert failed: {:?}", e)).into_response()
        }
    }
}

/// PUT /api/anon_users/{id}
pub async fn update_anon_user(
    Path(id): Path<Uuid>,
    Json(update): Json<AnonUserUpdate>,
) -> impl IntoResponse {
    let supa = Supabase::new_from_env().unwrap();
    let payload = serde_json::to_value(&update).unwrap();

    let result = supa
        .from("anon_users")
        .eq("id", &id.to_string())
        .update(payload)
        .select("*")
        .execute_typed::<AnonUser>()
        .await;

    match result {
        Ok(rows) => Json(json!({ "updated": rows })).into_response(),
        Err(e) => {
            eprintln!("❌ Error updating anon_user {}: {:?}", id, e);
            (StatusCode::BAD_REQUEST, format!("Update failed: {e:?}")).into_response()
        }
    }
}

/// DELETE /api/anon_users/{id}
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
