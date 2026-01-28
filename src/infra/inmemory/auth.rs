use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::core::UserId;
use crate::shared::identity::auth_source::AuthSource;

/// --------------------------------------------------
/// In-memory AuthSource
/// --------------------------------------------------
///
/// - NOT secure
/// - NOT persistent
/// - Intended for dev / tests only
/// --------------------------------------------------

pub struct InMemoryAuthSource {
    users: Mutex<HashMap<String, (UserId, String)>>, // email -> (id, password)
}

impl Default for InMemoryAuthSource {
    fn default() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AuthSource for InMemoryAuthSource {
    async fn signup(
        &self,
        email: String,
        password: String,
    ) -> Result<UserId> {
        let mut users = self.users.lock().unwrap();

        if users.contains_key(&email) {
            return Err(anyhow!("User already exists"));
        }

        let user_id = UserId::new();
        users.insert(email, (user_id, password));

        Ok(user_id)
    }

    async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<UserId> {
        let users = self.users.lock().unwrap();

        let (user_id, stored_password) = users
            .get(&email)
            .ok_or_else(|| anyhow!("User not found"))?;

        if *stored_password != password {
            return Err(anyhow!("Invalid credentials"));
        }

        Ok(*user_id)
    }
}
