use crate::supabasic::error::Result;
use crate::supabasic::{Supabase, SupabasicError};
use serde::{Deserialize};
use serde_json::json;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: Option<String>,
    pub category_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
impl Supabase {
    /// Create a new entity
    pub async fn create_entity(&self, name: &str) -> Result<Uuid> {
        let payload = json!([{ "name": name }]);

        let rows: Vec<Entity> = self
            .from("entities")
            .insert(payload)
            .select("id, name")
            .execute_typed()
            .await?;

        rows.first()
            .map(|e| e.id)
            .ok_or_else(|| SupabasicError::Other("No entity returned".into()))
    }

    /// Fetch all *active* entities (deleted_at is null)
    pub async fn fetch_entities(&self) -> Result<Vec<Entity>> {
        self.from("entities")
            .select("id, name, category_id, created_at, updated_at, deleted_at")
            .is_null("deleted_at")   // ✅ simpler and correct
            .execute_typed()
            .await
    }

    /// Fetch a single entity by id, but only if not deleted
    pub async fn fetch_entity_by_id(&self, id: Uuid) -> Result<Option<Entity>> {
        let rows: Vec<Entity> = self
            .from("entities")
            .select("id, name, category_id, created_at, updated_at, deleted_at")
            .eq("id", &id.to_string())
            .is_null("deleted_at")
            .execute_typed()
            .await?;

        Ok(rows.into_iter().next())
    }

    /// Fetch a single entity by id (raw, includes deleted ones)
    pub async fn fetch_entity_raw_by_id(&self, id: Uuid) -> Result<Option<Entity>> {
        let rows: Vec<Entity> = self
            .from("entities")
            .select("id, name, category_id, created_at, updated_at, deleted_at")
            .eq("id", &id.to_string())
            .execute_typed()
            .await?;

        Ok(rows.into_iter().next())
    }

    /// Soft delete: set deleted_at timestamp
    pub async fn soft_delete_entity(&self, id: Uuid) -> Result<bool> {
        // TODO: Fix soft delete — currently not updating deleted_at in DB
        let payload = serde_json::json!({ "deleted_at": chrono::Utc::now() });

        let rows: Vec<Entity> = self
            .from("entities")
            .update(payload)
            .eq("id", &id.to_string())
            .select("id, deleted_at")
            .execute_typed()
            .await?;

        Ok(!rows.is_empty())
    }



}
