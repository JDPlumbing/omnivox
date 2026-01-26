use async_trait::async_trait;
use anyhow::{Result, bail, anyhow};
use crate::core::UserId;
use crate::shared::identity::auth_source::AuthSource;
use crate::supabasic::Supabase;

/// Infra implementation backed by Supabase Auth
pub struct SupabaseAuthSource {
    supa: Supabase,
}

impl SupabaseAuthSource {
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
impl AuthSource for SupabaseAuthSource {
    async fn signup(&self, email: String, password: String) -> Result<UserId> {
        let resp = self.supa.auth_signup(&email, &password).await?;

        let user_id = resp
            .user_id
            .ok_or_else(|| anyhow!("Supabase returned no user id"))?;

        Ok(UserId::from_uuid(user_id))
    }



    async fn login(&self, email: String, password: String) -> Result<UserId> {
        let resp = self
            .supa
            .auth_login(&email, &password)
            .await?;

        let Some(user_id) = resp.user_id else {
            bail!("Supabase login succeeded but returned no user_id");
        };

        Ok(UserId::from_uuid(user_id))
    }
}
