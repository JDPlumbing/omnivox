// shared/permissions/permissions_source.rs
use async_trait::async_trait;
use anyhow::Result;

use crate::core::UserId;
use crate::core::WorldId;

#[derive(Debug, Clone)]
pub enum Action {
    View,
    Edit,
    Delete,
    Simulate,
    Admin,
}

#[derive(Debug, Clone)]
pub enum Resource {
    World(WorldId),
    Property,
    User(UserId),
}

#[async_trait]
pub trait PermissionsSource: Send + Sync {
    async fn can(
        &self,
        user_id: UserId,
        action: Action,
        resource: Resource,
    ) -> Result<bool>;
}
