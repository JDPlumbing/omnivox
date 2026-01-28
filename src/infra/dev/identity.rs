use async_trait::async_trait;
use anyhow::Result;

use crate::core::UserId;
use crate::shared::identity::auth_context::AccountRole;
use crate::shared::identity::identity_source::IdentitySource;

#[derive(Debug)]
pub struct DevIdentitySource {
    user_id: UserId,
    role: AccountRole,
}

impl DevIdentitySource {
    pub fn new(user_id: UserId, role: AccountRole) -> Self {
        Self { user_id, role }
    }
}

#[async_trait]
impl IdentitySource for DevIdentitySource {
    async fn lookup_by_external_id(
        &self,
        _external_id: &str,
    ) -> Result<Option<(UserId, AccountRole)>> {
        Ok(Some((self.user_id, self.role)))
    }

    async fn create_mapping(
        &self,
        _: &str,
        _: &UserId,
        _: AccountRole,
    ) -> Result<()> {
        Ok(())
    }
}
