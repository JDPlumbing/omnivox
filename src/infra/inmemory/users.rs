use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;

use crate::core::UserId;
use crate::shared::users::user_source::{UserSource, UserRecord};

/// --------------------------------------------------
/// In-memory UserSource
/// --------------------------------------------------

pub struct InMemoryUserSource {
    users: Mutex<HashMap<UserId, UserRecord>>,
}

impl Default for InMemoryUserSource {
    fn default() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserSource for InMemoryUserSource {
    async fn get_user(
        &self,
        user_id: UserId,
    ) -> Result<Option<UserRecord>> {
        let map = self.users.lock().unwrap();
        Ok(map.get(&user_id).cloned())
    }

    async fn list_users(&self) -> Result<Vec<UserRecord>> {
        let map = self.users.lock().unwrap();
        Ok(map.values().cloned().collect())
    }

    async fn create_user(
        &self,
        user_id: UserId,
        display_name: String,
    ) -> Result<()> {
        let mut map = self.users.lock().unwrap();

        map.insert(
            user_id,
            UserRecord {
                id: user_id,
                display_name: Some(display_name),
                role: None,
            },
        );

        Ok(())
    }
}
