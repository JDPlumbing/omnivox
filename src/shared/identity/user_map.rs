use serde_json::json;
use anyhow::{Result, anyhow};
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::core::id::user_id::UserId;

// --------------------------------------------------
// Lookup existing mapping
// --------------------------------------------------

pub async fn lookup_user_map(
    supa: &Supabase,
    supabase_id: &str,
) -> Option<(UserId, String)> {
    let res = supa
        .from("user_map")
        .select("user_id, role")
        .eq("supabase_id", supabase_id)
        .execute()
        .await
        .ok()?;

    let rows: Vec<serde_json::Value> =
        serde_json::from_value(res).ok()?;

    let row = rows.first()?;

    let user_id = row
        .get("user_id")?
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(UserId)?;

    let role = row
        .get("role")?
        .as_str()?
        .to_string();

    Some((user_id, role))
}

// --------------------------------------------------
// Create new mapping
// --------------------------------------------------

pub async fn insert_user_map(
    supa: &Supabase,
    supabase_id: &str,
    user_id: &UserId,
    role: &str,
) {
    let _ = supa
        .from("user_map")
        .insert(json!({
            "supabase_id": supabase_id,
            "user_id": user_id.0.to_string(),
            "role": role
        }))
        .execute()
        .await;
}

// --------------------------------------------------
// Canonical identity resolver
// --------------------------------------------------

pub async fn map_or_create_user(
    supa: &Supabase,
    sb_user: &serde_json::Value,
) -> Result<(UserId, String)> {

    let supabase_id = sb_user
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Supabase user missing id"))?;

    // 1. Try existing mapping
    if let Some((user_id, role)) =
        lookup_user_map(supa, supabase_id).await
    {
        return Ok((user_id, role));
    }

    // 2. Create new user
    let user_id = UserId::new();
    let role = "user".to_string();

    insert_user_map(
        supa,
        supabase_id,
        &user_id,
        &role,
    ).await;

    Ok((user_id, role))
}
