// infra/inmemory/identity.rs
use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Result;
use async_trait::async_trait;

use crate::core::UserId;
use crate::shared::identity::identity_source::IdentitySource;
use crate::shared::identity::auth_context::AccountRole;

pub struct InMemoryIdentitySource {
    mappings: Mutex<HashMap<String, (UserId, AccountRole)>>,
}

impl Default for InMemoryIdentitySource {
    fn default() -> Self {
        Self {
            mappings: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl IdentitySource for InMemoryIdentitySource {
    async fn lookup_by_external_id(
        &self,
        external_id: &str,
    ) -> Result<Option<(UserId, AccountRole)>> {
        Ok(self.mappings.lock().unwrap().get(external_id).cloned())
    }

    async fn create_mapping(
        &self,
        external_id: &str,
        user_id: &UserId,
        role: AccountRole,
    ) -> Result<()> {
        self.mappings
            .lock()
            .unwrap()
            .insert(external_id.to_string(), (*user_id, role));
        Ok(())
    }
}
