use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::core::WorldId;
use crate::shared::session::session_context::SessionContext;

#[async_trait]
pub trait SessionSource: Send + Sync {
    async fn resume(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionContext>>;

    async fn create_anonymous(
        &self,
    ) -> Result<(Uuid, SessionContext)>;

    async fn set_world(
        &self,
        session_id: Uuid,
        world_id: WorldId,
    ) -> Result<()>;
}
