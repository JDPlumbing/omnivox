use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::core::{UserId, WorldId};
use crate::core::old_spatial::SpatialAnchor;
use crate::core::old_spatial::SpatialHorizon;
use crate::shared::session::session_context::SessionContext;
use crate::shared::session::session_source::SessionSource;

/// --------------------------------------------------
/// In-memory SessionSource
/// --------------------------------------------------

pub struct InMemorySessionSource {
    sessions: Mutex<HashMap<Uuid, SessionContext>>,
}

impl Default for InMemorySessionSource {
    fn default() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SessionSource for InMemorySessionSource {
    async fn resume(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionContext>> {
        let map = self.sessions.lock().unwrap();
        Ok(map.get(&session_id).cloned())
    }

    async fn get_session(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionContext>> {
        let map = self.sessions.lock().unwrap();
        Ok(map.get(&session_id).cloned())
    }

    async fn create_anonymous(
        &self,
    ) -> Result<(Uuid, SessionContext)> {
        let mut map = self.sessions.lock().unwrap();

        let session_id = Uuid::new_v4();
        let anon_owner_id = Uuid::new_v4();

        let ctx = SessionContext::anonymous(anon_owner_id);

        map.insert(session_id, ctx.clone());
        Ok((session_id, ctx))
    }

    async fn set_world(
        &self,
        session_id: Uuid,
        world_id: WorldId,
    ) -> Result<()> {
        let mut map = self.sessions.lock().unwrap();

        if let Some(ctx) = map.get_mut(&session_id) {
            ctx.world_id = Some(world_id);
        }

        Ok(())
    }

    async fn upgrade_to_user(
        &self,
        session_id: Uuid,
        user_id: UserId,
    ) -> Result<()> {
        let mut map = self.sessions.lock().unwrap();

        if let Some(ctx) = map.get_mut(&session_id) {
            *ctx = SessionContext::authenticated(user_id);
        }

        Ok(())
    }

    async fn set_spatial_anchor(
        &self,
        session_id: Uuid,
        anchor: SpatialAnchor,
    ) -> Result<()> {
        let mut map = self.sessions.lock().unwrap();

        if let Some(ctx) = map.get_mut(&session_id) {
            ctx.spatial_anchor = Some(anchor);
        }

        Ok(())
    }

    async fn set_spatial_horizon(
        &self,
        session_id: Uuid,
        horizon: SpatialHorizon,
    ) -> Result<()> {
        let mut map = self.sessions.lock().unwrap();

        if let Some(ctx) = map.get_mut(&session_id) {
            ctx.spatial_horizon = Some(horizon);
        }

        Ok(())
    }
}
