use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use crate::shared::users::anon_user_source::AnonUserSource;
use crate::supabasic::Supabase;
use crate::core::UserId;
use crate::shared::session::{
    session_context::SessionContext,
    session_source::SessionSource,
};
use crate::core::WorldId;
use crate::core::spatial::SpatialAnchor;
use crate::core::SpatialHorizon;

pub struct SupabaseSessionSource {
    supa: Supabase,
    anon_user_source: Arc<dyn AnonUserSource>,
}

impl SupabaseSessionSource {
    pub fn new(supa: Supabase, anon_user_source: Arc<dyn AnonUserSource>) -> Self {
        Self { supa, anon_user_source }
    }

    pub fn new_from_env() -> anyhow::Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
            anon_user_source: Arc::new(crate::infra::users::supabase_anon_user_source::SupabaseAnonUserSource::new_from_env()?),
        })
    }
}

#[async_trait]
impl SessionSource for SupabaseSessionSource {
   async fn resume(
    &self,
    session_id: Uuid,
) -> Result<Option<SessionContext>> {
    let row = self
        .supa
        .from("anon_sessions")
        .select("engine_user_id, anon_owner_id, world_id, spatial_anchor, spatial_horizon")
        .eq("session_id", &session_id.to_string())
        .maybe_single_typed::<serde_json::Value>()
        .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let engine_user_id = row
        .get("engine_user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| anyhow::anyhow!("Missing engine_user_id"))?;

    let anon_owner_id = row
        .get("anon_owner_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    let user_id = Some(UserId::from_uuid(engine_user_id));

    let world_id = row
        .get("world_id")
        .and_then(|v| v.as_i64())
        .map(WorldId);

    let is_anon = match anon_owner_id {
        Some(id) => !self.anon_user_source.is_upgraded(id).await?,
        None => false,
    };
    let spatial_anchor = row
        .get("spatial_anchor")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    let spatial_horizon = row
        .get("spatial_horizon")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    Ok(Some(SessionContext {
        user_id,
        world_id,
        property_id: None,
        is_anon,
        anon_owner_id,
        spatial_anchor,
        spatial_horizon,
    }))
}




    async fn create_anonymous(
        &self,
    ) -> Result<(Uuid, SessionContext)> {
        let session_id = Uuid::new_v4();
        let engine_user_id = UserId::new();
        let anon_owner_id = Uuid::new_v4();

        // Create anon user
        let _ = self
            .supa
            .from("anon_users")
            .insert(serde_json::json!({
                "id": anon_owner_id
            }))
            .execute()
            .await?;

        // Create session
        let _ = self
            .supa
            .from("anon_sessions")
            .insert(serde_json::json!({
                "session_id": session_id,
                "engine_user_id": engine_user_id.to_string(),
                "anon_owner_id": anon_owner_id,
                "world_id": null
            }))
            .execute()
            .await?;

        Ok((
            session_id,
            SessionContext {
                user_id: Some(engine_user_id),
                world_id: None,
                property_id: None,
                is_anon: true,
                anon_owner_id: Some(anon_owner_id),
                spatial_anchor: None,
                spatial_horizon: None,
            },

        ))
    }

    async fn set_world(
        &self,
        session_id: Uuid,
        world_id: WorldId,
    ) -> Result<()> {
        let _ = self
            .supa
            .from("anon_sessions")
            .update(serde_json::json!({
                "world_id": world_id.0
            }))
            .eq("session_id", &session_id.to_string())
            .execute()
            .await?;

        Ok(())
    }
    
async fn get_session(
    &self,
    session_id: Uuid,
) -> Result<Option<SessionContext>> {
    let row = self
        .supa
        .from("anon_sessions")
        .select("engine_user_id, anon_owner_id, world_id, spatial_anchor, spatial_horizon")

        .eq("session_id", &session_id.to_string())
        .maybe_single_typed::<serde_json::Value>()
        .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let user_id = row
        .get("engine_user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(UserId::from_uuid);

    let world_id = row
        .get("world_id")
        .and_then(|v| v.as_i64())
        .map(WorldId);

    let anon_owner_id = row
        .get("anon_owner_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    let is_anon = match anon_owner_id {
        Some(id) => !self.anon_user_source.is_upgraded(id).await?,
        None => false,
    };
    let spatial_anchor = row
        .get("spatial_anchor")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    let spatial_horizon = row
        .get("spatial_horizon")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    Ok(Some(SessionContext {
        user_id,
        world_id,
        property_id: None,
        is_anon,
        anon_owner_id,
        spatial_anchor,
        spatial_horizon,
    }))

}

async fn upgrade_to_user(
    &self,
    session_id: Uuid,
    user_id: UserId,
) -> Result<()> {
    self.supa
        .from("anon_sessions")
        .update(serde_json::json!({
            "engine_user_id": user_id.to_string()
        }))
        .eq("session_id", &session_id.to_string())
        .execute()
        .await?;

    Ok(())
}

async fn set_spatial_anchor(
    &self,
    session_id: Uuid,
    anchor: SpatialAnchor,
) -> Result<()> {
    self.supa
        .from("anon_sessions")
        .update(serde_json::json!({
            "spatial_anchor": anchor
        }))
        .eq("session_id", &session_id.to_string())
        .execute()
        .await?;

    Ok(())
}
async fn set_spatial_horizon(
    &self,
    session_id: Uuid,
    horizon: SpatialHorizon,
) -> Result<()> {
    self.supa
        .from("anon_sessions")
        .update(serde_json::json!({
            "spatial_horizon": serde_json::to_value(horizon)?
        }))
        .eq("session_id", &session_id.to_string())
        .execute()
        .await?;

    Ok(())
}


}