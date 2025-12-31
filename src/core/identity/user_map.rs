use serde_json::json;
use crate::shared::app_state::AppState;
use crate::core::id::user_id::UserId;

// ----------------------------
// Helpers
// ----------------------------

fn supabase_user_id(sb_user: &serde_json::Value) -> Option<&str> {
    sb_user.get("id")?.as_str()
}

// ----------------------------
// Lookup existing mapping
// ----------------------------

pub async fn lookup_user_map(
    app: &AppState,
    supabase_id: &str,
) -> Option<(String, String)> {
    let res = app
        .supa
        .from("user_map")
        .select("user_id, role")
        .eq("supabase_id", supabase_id)
        .execute()
        .await
        .ok()?;

    let rows: Vec<serde_json::Value> =
        serde_json::from_value(res).ok()?;

    let row = rows.first()?;

    Some((
        row.get("user_id")?.as_str()?.to_string(),
        row.get("role")?.as_str()?.to_string(),
    ))
}

// ----------------------------
// Create new mapping
// ----------------------------

pub async fn insert_user_map(
    app: &AppState,
    supabase_id: &str,
    user_id: &UserId,
    role: &str,
) {
    let _ = app
        .supa
        .from("user_map")
        .insert(json!({
            "supabase_id": supabase_id,
            "user_id": user_id.to_string(),
            "role": role
        }))
        .execute()
        .await;
}

// ----------------------------
// Canonical identity resolver
// ----------------------------

use anyhow::{Result, anyhow};

pub async fn map_or_create_user(
    app: &AppState,
    sb_user: &serde_json::Value,
) -> Result<(UserId, String)> {

    let sb_id = sb_user
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Supabase user missing id"))?;

    // TODO: lookup or create internal user mapping here
    let user_id = UserId::from_string(sb_id);
    let role = "user".to_string();

    Ok((user_id, role))
}
