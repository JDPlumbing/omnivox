use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::core::UserId;
use crate::shared::users::anon_user_source::{AnonUserSource, AnonUserRecord};

/// --------------------------------------------------
/// In-memory AnonUserSource
/// --------------------------------------------------

pub struct InMemoryAnonUserSource {
    anon_users: Mutex<HashMap<Uuid, AnonUserRecord>>,
    upgraded: Mutex<HashMap<Uuid, UserId>>,
}

impl Default for InMemoryAnonUserSource {
    fn default() -> Self {
        Self {
            anon_users: Mutex::new(HashMap::new()),
            upgraded: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AnonUserSource for InMemoryAnonUserSource {
    async fn get_anon_user(
        &self,
        id: Uuid,
    ) -> Result<Option<AnonUserRecord>> {
        let map = self.anon_users.lock().unwrap();
        Ok(map.get(&id).cloned())
    }

    async fn list_anon_users(
        &self,
    ) -> Result<Vec<AnonUserRecord>> {
        let map = self.anon_users.lock().unwrap();
        Ok(map.values().cloned().collect())
    }

    async fn create_anon_user(
        &self,
        display_name: Option<String>,
    ) -> Result<AnonUserRecord> {
        let mut map = self.anon_users.lock().unwrap();

        let record = AnonUserRecord {
            id: Uuid::new_v4(),
            display_name,
        };

        map.insert(record.id, record.clone());
        Ok(record)
    }

    async fn delete_anon_user(
        &self,
        id: Uuid,
    ) -> Result<()> {
        let mut map = self.anon_users.lock().unwrap();
        map.remove(&id);
        Ok(())
    }

    async fn mark_upgraded(
        &self,
        anon_user_id: Uuid,
        real_user_id: UserId,
    ) -> Result<()> {
        self.upgraded
            .lock()
            .unwrap()
            .insert(anon_user_id, real_user_id);
        Ok(())
    }

    async fn is_upgraded(
        &self,
        anon_user_id: Uuid,
    ) -> Result<bool> {
        Ok(self.upgraded.lock().unwrap().contains_key(&anon_user_id))
    }
}
