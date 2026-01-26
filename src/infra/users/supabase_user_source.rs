// infra/users/supabase_user_source.rs
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use serde_json::Value;
use crate::supabasic::Supabase;
use crate::core::UserId;
use crate::shared::users::user_source::{UserSource, UserRecord};

pub struct SupabaseUserSource {
    supa: Supabase,
}

impl SupabaseUserSource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
        })
    }
}

#[async_trait]
impl UserSource for SupabaseUserSource {
    async fn get_user(
        &self,
        user_id: UserId,
    ) -> Result<Option<UserRecord>> {
        let id = user_id.to_string();

        let row = self
            .supa
            .from("users")
            .select("id,display_name,role")
            .eq("id", &id)
            .maybe_single_typed::<serde_json::Value>()
            .await?;

        Ok(row.map(|r| UserRecord {
            id: user_id,
            display_name: r["display_name"].as_str().map(str::to_string),
            role: r["role"].as_str().map(str::to_string),
        }))
    }

    async fn list_users(&self) -> Result<Vec<UserRecord>> {
        let rows = self
            .supa
            .from("users")
            .select("id,display_name,role")
            .execute()
            .await?;

        let users = rows
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|r| {
                let id = r["id"].as_str()?;
                Some(UserRecord {
                    id: UserId::from_uuid(Uuid::parse_str(id).ok()?),
                    display_name: r["display_name"].as_str().map(str::to_string),
                    role: r["role"].as_str().map(str::to_string),
                })
            })
            .collect();

        Ok(users)
    }
async fn create_user(
    &self,
    user_id: UserId,
    display_name: String,
) -> Result<()> {

    // 1️⃣ Check if user already exists
    let existing = self
        .supa
        .from("users")
        .select("id")
        .eq("id", &user_id.to_string())
        .maybe_single_typed::<serde_json::Value>()
        .await?;

    if existing.is_some() {
        // User already exists → idempotent success
        return Ok(());
    }

    // 2️⃣ Insert new user
    self.supa
        .from("users")
        .insert(serde_json::json!({
            "id": &user_id.to_string(),
            "display_name": display_name,
            "role": "customer",
        }))
        .execute()
        .await?;

    Ok(())
}

}
