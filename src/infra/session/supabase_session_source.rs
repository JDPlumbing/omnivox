use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::core::UserId;
use crate::shared::session::{
    session_context::SessionContext,
    session_source::SessionSource,
};
use crate::core::WorldId;

pub struct SupabaseSessionSource {
    supa: Supabase,
}

impl SupabaseSessionSource {
    pub fn new(supa: Supabase) -> Self {
        Self { supa }
    }

    pub fn new_from_env() -> anyhow::Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
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
            .select("engine_user_id, world_id")
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


        Ok(Some(SessionContext {
            user_id,
            world_id,
            property_id: None,
            is_anon: true,
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

}
