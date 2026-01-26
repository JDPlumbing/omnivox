use anyhow::{Result, anyhow};
use uuid::Uuid;

use crate::supabasic::Supabase;
use crate::core::UserId;
use crate::shared::identity::{
    auth_context::AccountRole,
    identity_source::{IdentitySource, ResolvedIdentity},
};

pub struct SupabaseIdentitySource {
    supa: Supabase,
}

impl SupabaseIdentitySource {
    pub fn new_from_env() -> Result<Self> {
        Ok(Self {
            supa: Supabase::new_from_env()?,
        })
    }
}

#[async_trait::async_trait]
impl IdentitySource for SupabaseIdentitySource {
    async fn resolve_from_token(
        &self,
        token: &str,
    ) -> Result<ResolvedIdentity> {
        let user = self
            .supa
            .get_user_from_jwt(token.to_string())
            .await
            .map_err(|_| anyhow!("invalid token"))?;

        let supabase_user_id = Uuid::parse_str(
            user["id"]
                .as_str()
                .ok_or_else(|| anyhow!("missing supabase id"))?,
        )?;

        let user_id = UserId::from_uuid(supabase_user_id);

        let role = if std::env::var("ROOT_USER_ID")
            .map(|v| v == supabase_user_id.to_string())
            .unwrap_or(false)
        {
            AccountRole::Root
        } else {
            AccountRole::User
        };

        Ok(ResolvedIdentity { user_id, role })
    }
}
