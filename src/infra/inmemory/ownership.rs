use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;

use crate::core::id::user_id::UserId;
use crate::core::ownership::Ownership;
use crate::shared::ownership::ownership_source::OwnershipSource;

/// --------------------------------------------------
/// In-memory OwnershipSource
/// --------------------------------------------------

/// Keyed by (user_id, resource_id)
pub struct InMemoryOwnershipSource {
    ownerships: Mutex<HashMap<(UserId, String), Ownership>>,
}

impl Default for InMemoryOwnershipSource {
    fn default() -> Self {
        Self {
            ownerships: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl OwnershipSource for InMemoryOwnershipSource {
    async fn list_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<Ownership>> {
        let map = self.ownerships.lock().unwrap();

        Ok(map
            .values()
            .filter(|o| o.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn get(
        &self,
        user_id: UserId,
        resource_id: String,
    ) -> Result<Option<Ownership>> {
        let map = self.ownerships.lock().unwrap();
        Ok(map.get(&(user_id, resource_id)).cloned())
    }

    async fn create(
        &self,
        ownership: Ownership,
    ) -> Result<Ownership> {
        let mut map = self.ownerships.lock().unwrap();

        let key = (ownership.user_id, ownership.resource_id.clone());
        map.insert(key, ownership.clone());

        Ok(ownership)
    }

    async fn delete(
        &self,
        user_id: UserId,
        resource_id: String,
    ) -> Result<()> {
        let mut map = self.ownerships.lock().unwrap();
        map.remove(&(user_id, resource_id));
        Ok(())
    }
}
