// infra/users/supabase_anon_user_source.rs
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::core::UserId;

use crate::supabasic::Supabase;
use crate::shared::users::anon_user_source::{
    AnonUserSource,
    AnonUserRecord,
};

pub struct SupabaseAnonUserSource {
    supa: Supabase,
}

impl SupabaseAnonUserSource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
        })
    }
}

#[async_trait]
impl AnonUserSource for SupabaseAnonUserSource {
    async fn get_anon_user(
        &self,
        id: Uuid,
    ) -> Result<Option<AnonUserRecord>> {
        let row = self
            .supa
            .from("anon_users")
            .select("id,display_name")
            .eq("id",   &id.to_string())
            .maybe_single_typed::<serde_json::Value>()
            .await?;

        Ok(row.map(|r| AnonUserRecord {
            id,
            display_name: r["display_name"]
                .as_str()
                .map(str::to_string),
        }))
    }

    async fn list_anon_users(
        &self,
    ) -> Result<Vec<AnonUserRecord>> {
        let rows = self
            .supa
            .from("anon_users")
            .select("id,display_name")
            .execute()
            .await?;

        let users = rows
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|r| {
                let id = Uuid::parse_str(r["id"].as_str()?).ok()?;
                Some(AnonUserRecord {
                    id,
                    display_name: r["display_name"]
                        .as_str()
                        .map(str::to_string),
                })
            })
            .collect();

        Ok(users)
    }

    async fn create_anon_user(
        &self,
        display_name: Option<String>,
    ) -> Result<AnonUserRecord> {
        let row = self
            .supa
            .from("anon_users")
            .insert(serde_json::json!({
                "display_name": display_name
            }))
            .select("id,display_name")
            .single()
            .await?;

        let id = Uuid::parse_str(
            row["id"].as_str().unwrap(),
        )?;

        Ok(AnonUserRecord {
            id,
            display_name: row["display_name"]
                .as_str()
                .map(str::to_string),
        })
    }

    async fn delete_anon_user(
        &self,
        id: Uuid,
    ) -> Result<()> {
        self
            .supa
            .from("anon_users")
            .eq("id", &id.to_string())
            .delete()
            .execute()
            .await?;

        Ok(())
    }

    async fn mark_upgraded(
        &self,
        anon_user_id: Uuid,
        real_user_id: UserId,
    ) -> Result<()> {
        self.supa
            .from("anon_users")
            .update(serde_json::json!({
                "upgraded_to_user_id": real_user_id.to_string(),
                "upgraded_at": chrono::Utc::now(),
            }))
            .eq("id", &anon_user_id.to_string())
            .execute()
            .await?;

        Ok(())
    }

    async fn is_upgraded(&self, anon_user_id: Uuid) -> Result<bool> {
    let row = self
        .supa
        .from("anon_users")
        .select("upgraded_to_user_id")
        .eq("id", &anon_user_id.to_string())
        .single_typed::<serde_json::Value>()
        .await?;

    Ok(row.get("upgraded_to_user_id").is_some())
}

}
