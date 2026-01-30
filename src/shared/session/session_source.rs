use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::core::{WorldId, UserId};
use crate::shared::session::session_context::SessionContext;
use crate::core::old_spatial::SpatialAnchor;
use crate::core::old_spatial::SpatialHorizon;

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

    async fn get_session(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionContext>>;

    async fn upgrade_to_user(
        &self,
        session_id: Uuid,
        user_id: UserId,
    ) -> Result<()>;

    async fn set_spatial_anchor(
        &self,
        session_id: Uuid,
        anchor: SpatialAnchor,
    ) -> Result<()>;

    async fn set_spatial_horizon(
        &self,
        session_id: Uuid,
        horizon: SpatialHorizon,
    ) -> Result<()>;
}
