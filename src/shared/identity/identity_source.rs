use anyhow::Result;
use crate::core::UserId;
use crate::shared::identity::auth_context::AccountRole;

#[async_trait::async_trait]
pub trait IdentitySource: Send + Sync {
    async fn lookup_by_external_id(
        &self,
        external_id: &str,
    ) -> Result<Option<(UserId, AccountRole)>>;

    async fn create_mapping(
        &self,
        external_id: &str,
        user_id: &UserId,
        role: AccountRole,
    ) -> Result<()>;
}
