use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use serde_json::Value;

use crate::supabasic::Supabase;
use crate::supabasic::properties::PropertyRecord;

use crate::core::{
    UserId,
    WorldId,
    Property,
    CreateProperty,
    UpdateProperty,
};

use crate::shared::properties::property_source::{
    PropertySource,
    PropertySummary,
};

pub struct SupabasePropertySource {
    supa: Supabase,
}

impl SupabasePropertySource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
        })
    }
}

#[async_trait]
impl PropertySource for SupabasePropertySource {
    // --------------------------------------------------
    // Queries
    // --------------------------------------------------

    async fn list_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<PropertySummary>> {
        let rows = self
            .supa
            .from("properties")
            .select("property_id, world_id, user_owner_id")
            .eq("user_owner_id", &user_id.to_string())
            .execute()
            .await?;

        let rows: Vec<Value> = serde_json::from_value(rows)?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some(PropertySummary {
                    property_id: row.get("property_id")?.as_str()?.parse().ok()?,
                    world_id: WorldId(row.get("world_id")?.as_i64()?),
                    owner_id: user_id,
                })
            })
            .collect())
    }

    async fn get(
        &self,
        property_id: Uuid,
    ) -> Result<Option<Property>> {
        let row = self
            .supa
            .from("properties")
            .select("*")
            .eq("property_id", &property_id.to_string())
            .maybe_single_typed()
            .await?;

        match row {
            Some(value) => {
                let record: PropertyRecord = serde_json::from_value(value)?;
                Ok(Some(Property::from(record)))
            }
            None => Ok(None),
        }
    }

    async fn list_for_world(
        &self,
        world_id: WorldId,
    ) -> Result<Vec<Property>> {
        let rows = self
            .supa
            .from("properties")
            .select("*")
            .eq("world_id", &world_id.0.to_string())
            .execute()
            .await?;

        let rows: Vec<Value> = serde_json::from_value(rows)?;

        Ok(rows
            .into_iter()
            .filter_map(|value| {
                let record: PropertyRecord = serde_json::from_value(value).ok()?;
                Some(Property::from(record))
            })
            .collect())
    }

    async fn list_all(&self) -> Result<Vec<Property>> {
        let rows = self
            .supa
            .from("properties")
            .select("*")
            .execute()
            .await?;

        let rows: Vec<Value> = serde_json::from_value(rows)?;

        Ok(rows
            .into_iter()
            .filter_map(|value| {
                let record: PropertyRecord = serde_json::from_value(value).ok()?;
                Some(Property::from(record))
            })
            .collect())
    }

    // --------------------------------------------------
    // Commands
    // --------------------------------------------------

    async fn create(
        &self,
        cmd: CreateProperty,
    ) -> Result<Property> {
        let record: PropertyRecord = cmd.into();

        let inserted = self
            .supa
            .from("properties")
            .insert(record)
            .select("*")
            .single()
            .await?;

        let record: PropertyRecord = serde_json::from_value(inserted)?;
        Ok(Property::from(record))
    }

    async fn update(
        &self,
        cmd: UpdateProperty,
    ) -> Result<Property> {
        let record = PropertyRecord::update(
            &self.supa,
            cmd.property_id,
            &cmd.into(),
        )
        .await?;

        Ok(Property::from(record))
    }

    async fn delete(
        &self,
        property_id: Uuid,
    ) -> Result<()> {
        self.supa
            .from("properties")
            .delete()
            .eq("property_id", &property_id.to_string())
            .execute()
            .await?;

        Ok(())
    }
}
