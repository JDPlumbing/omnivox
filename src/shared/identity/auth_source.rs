use async_trait::async_trait;
use anyhow::Result;
use crate::core::UserId;

#[async_trait]
pub trait AuthSource: Send + Sync {
    async fn signup(
        &self,
        email: String,
        password: String,
    ) -> Result<UserId>;

    async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<UserId>;
}
