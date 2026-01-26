use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::{Result, bail, anyhow};
use tokio::sync::RwLock;

use crate::core::id::WorldId;
use crate::core::UserId;
use crate::engine::world::state::WorldState;
use crate::shared::world_sources::source::WorldSource;
use crate::shared::session::session_source::SessionSource;

pub struct WorldEngine {
    /// Session management
    pub session_source: Arc<dyn crate::shared::session::session_source::SessionSource + Send + Sync>,

    /// Persistent world loader (infra-backed)
    pub world_source: Arc<dyn WorldSource + Send + Sync>,

    /// In-memory active worlds (runtime authority)
    pub worlds: Arc<RwLock<HashMap<WorldId, Arc<RwLock<WorldState>>>>>,
}
const EARTH_ID: WorldId = WorldId(1);
impl WorldEngine {
pub async fn enter_world(
    &self,
    session_id: Uuid,
    world_id: WorldId,
) -> Result<WorldState> {

    // 1️⃣ Load session
    let session = self
        .session_source
        .get_session(session_id)
        .await?
        .ok_or_else(|| anyhow!("Session not found"))?;

    // 2️⃣ Load world metadata
    let world = self
        .world_source
        .get_world(world_id)
        .await?;

    // 3️⃣ (Future) Permission checks
    // if !can_enter(session, world) { bail!("Forbidden") }

    // 4️⃣ Attach world to session
    self.session_source
        .set_world(session_id, world_id)
        .await?;

    // 5️⃣ Load world state
    let state = self
        .world_source
        .load_world(world_id)
        .await?;

    Ok(state)
}
pub async fn ensure_world(
    &self,
    session_id: Uuid,
) -> Result<WorldId> {
    let session = self
        .session_source
        .get_session(session_id)
        .await?
        .ok_or_else(|| anyhow!("Session not found"))?;

    let world_id = match session.world_id {
        Some(id) => id,
        None => {
            self.session_source
                .set_world(session_id, EARTH_ID)
                .await?;
            EARTH_ID
        }
    };

    Ok(world_id)
}


}


