use async_trait::async_trait;
use anyhow::Result;

use crate::core::UserId;
use crate::shared::identity::auth_source::AuthSource;

pub struct DevAuthSource {
    user_id: UserId,
}

impl DevAuthSource {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}

#[async_trait]
impl AuthSource for DevAuthSource {
    async fn signup(&self, _email: String, _password: String) -> Result<UserId> {
        Ok(self.user_id)
    }

    async fn login(&self, _email: String, _password: String) -> Result<UserId> {
        Ok(self.user_id)
    }
}
