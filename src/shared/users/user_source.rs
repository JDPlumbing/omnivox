// shared/users/user_source.rs
use async_trait::async_trait;
use anyhow::Result;
use crate::core::UserId;

#[derive(Debug, Clone)]
pub struct UserRecord {
    pub id: UserId,
    pub display_name: Option<String>,
    pub role: Option<String>,
}


#[async_trait]
pub trait UserSource: Send + Sync {
    async fn get_user(&self, user_id: UserId) -> Result<Option<UserRecord>>;

    async fn list_users(&self) -> Result<Vec<UserRecord>>;

    async fn create_user(&self, user_id: UserId, display_name: String) -> Result<()>;
}
