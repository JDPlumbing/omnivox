use anyhow::{Result, bail, anyhow};
use uuid::Uuid;
use std::sync::Arc;

use crate::core::UserId;
use crate::shared::{
    session::session_source::SessionSource,
    users::user_source::UserSource,
    identity::auth_source::AuthSource,
    users::anon_user_source::AnonUserSource,
};

pub struct UserEngine {
    pub session_source: Arc<dyn SessionSource + Send + Sync>,
    pub auth_source: Arc<dyn AuthSource + Send + Sync>,
    pub user_source: Arc<dyn UserSource + Send + Sync>,
    pub anon_user_source: Arc<dyn AnonUserSource + Send + Sync>,
}

impl UserEngine {
    pub fn new(
        auth_source: Arc<dyn AuthSource + Send + Sync>,
        user_source: Arc<dyn UserSource + Send + Sync>,
        anon_user_source: Arc<dyn AnonUserSource + Send + Sync>,
        session_source: Arc<dyn SessionSource + Send + Sync>,
    ) -> Self {
        Self {
            auth_source,
            user_source,
            anon_user_source,
            session_source,
        }
    }

    pub async fn signup(
        &self,
        session_id: Uuid,
        email: String,
        password: String,
        display_name: String,
    ) -> Result<UserId> {
        // 1️⃣ Load session
        let session = self
            .session_source
            .get_session(session_id)
            .await?
            .ok_or_else(|| anyhow!("Session not found"))?;

        if !session.is_anon {
            bail!("Session already associated with a real user");
        }

        let anon_user_id = session
            .anon_owner_id
            .ok_or_else(|| anyhow!("Session is not anonymous"))?;

        if self.anon_user_source.is_upgraded(anon_user_id).await? {
            bail!("Session already associated with a real user");
        }

        // 2️⃣ Create auth user
        let user_id = self
            .auth_source
            .signup(email, password)
            .await?;

        // 3️⃣ Create domain user
        self.user_source
            .create_user(user_id, display_name)
            .await?;

        // 4️⃣ Mark anon identity as upgraded
        self.anon_user_source
            .mark_upgraded(anon_user_id, user_id)
            .await?;

        // 5️⃣ Upgrade session identity
        self.session_source
            .upgrade_to_user(session_id, user_id)
            .await?;

        Ok(user_id)
    }
}
