use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::core::{WorldId};
use crate::core::{Property, CreateProperty, UpdateProperty};

#[async_trait]
pub trait PropertySource: Send + Sync {
    // ---- Queries ----

    async fn get(
        &self,
        property_id: Uuid,
    ) -> Result<Option<Property>>;

    async fn list_all(&self) -> Result<Vec<Property>>;

    async fn list_for_world(
        &self,
        world_id: WorldId,
    ) -> Result<Vec<Property>>;

    // ---- Commands ----

    async fn create(
        &self,
        input: &CreateProperty,
    ) -> Result<Property>;

    async fn update(
        &self,
        cmd: UpdateProperty,
    ) -> Result<Property>;

    async fn delete(
        &self,
        property_id: Uuid,
    ) -> Result<()>;
}
