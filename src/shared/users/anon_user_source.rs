// shared/users/anon_user_source.rs
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::core::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonUserRecord {
    pub id: Uuid,
    pub display_name: Option<String>,
}


#[async_trait]
pub trait AnonUserSource: Send + Sync {
    async fn get_anon_user(
        &self,
        id: Uuid,
    ) -> Result<Option<AnonUserRecord>>;

    async fn list_anon_users(
        &self,
    ) -> Result<Vec<AnonUserRecord>>;

    async fn create_anon_user(
        &self,
        display_name: Option<String>,
    ) -> Result<AnonUserRecord>;

    async fn delete_anon_user(
        &self,
        id: Uuid,
    ) -> Result<()>;


        async fn mark_upgraded(
            &self,
            anon_user_id: Uuid,
            real_user_id: UserId,
        ) -> Result<()>;

        async fn is_upgraded(
            &self,
            anon_user_id: Uuid,
        ) -> Result<bool>;
    }
