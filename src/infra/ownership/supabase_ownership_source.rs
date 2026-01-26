use async_trait::async_trait;
use anyhow::Result;

use crate::supabasic::Supabase;
use crate::core::{UserId, WorldId};
use crate::shared::ownership::ownership_source::{
    OwnershipSource,
    OwnershipContext,
};
use uuid::Uuid;

pub struct SupabaseOwnershipSource {
    supa: Supabase,
}

impl SupabaseOwnershipSource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
        })
    }
}



#[async_trait]
impl OwnershipSource for SupabaseOwnershipSource {
    async fn resolve_ownership(
        &self,
        user_id: UserId,
    ) -> Result<OwnershipContext> {
        let rows = self
            .supa
            .from("user_properties")
            .select("property_id, role, world_id")
            .eq("user_id", &user_id.to_string())
            .execute()
            .await?;

        let rows: Vec<serde_json::Value> = serde_json::from_value(rows)?;
        if let Some(row) = rows.first() {
            Ok(OwnershipContext {
                user_id,
                property_id: row
                    .get("property_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse().ok()),
                property_role: row
                    .get("role")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                world_id: row
                    .get("world_id")
                    .and_then(|v| v.as_i64())
                    .map(WorldId),
            })
        } else {
            Ok(OwnershipContext {
                user_id,
                property_id: None,
                property_role: None,
                world_id: None,
            })
        }
    }
}
