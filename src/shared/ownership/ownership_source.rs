// shared/ownership/ownership_source.rs
use async_trait::async_trait;
use anyhow::Result;

use crate::core::UserId;
use crate::core::WorldId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OwnershipContext {
    pub user_id: UserId,
    pub property_id: Option<Uuid>,
    pub property_role: Option<String>,
    pub world_id: Option<WorldId>,
}

#[async_trait]
pub trait OwnershipSource: Send + Sync {
    async fn resolve_ownership(
        &self,
        user_id: UserId,
    ) -> Result<OwnershipContext>;
}
