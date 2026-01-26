use anyhow::Result;
use crate::core::UserId;
use crate::shared::identity::auth_context::AccountRole;

#[derive(Debug, Clone)]
pub struct ResolvedIdentity {
    pub user_id: UserId,
    pub role: AccountRole,
}

#[async_trait::async_trait]
pub trait IdentitySource: Send + Sync {
    async fn resolve_from_token(
        &self,
        token: &str,
    ) -> Result<ResolvedIdentity>;
}
