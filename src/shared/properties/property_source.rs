use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::core::{UserId, WorldId};
use crate::core::{Property, CreateProperty, UpdateProperty};

#[derive(Debug, Clone)]
pub struct PropertySummary {
    pub property_id: Uuid,
    pub world_id: WorldId,
    pub owner_id: UserId,
}

#[async_trait]
pub trait PropertySource: Send + Sync {
    // ---- Queries ----

    async fn list_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<PropertySummary>>;

    async fn get(
        &self,
        property_id: Uuid,
    ) -> Result<Option<Property>>;
    async fn list_all(&self) -> Result<Vec<Property>>;
    // ---- Commands ----

    async fn create(
        &self,
        input: CreateProperty,
    ) -> Result<Property>;

    async fn delete(&self, property_id: Uuid) -> Result<()>;
    async fn update(&self, cmd: UpdateProperty) -> Result<Property>;
    async fn list_for_world(
        &self,
        world_id: WorldId,
    ) -> Result<Vec<Property>>;

}
