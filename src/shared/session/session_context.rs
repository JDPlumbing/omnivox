// shared/session/session_context.rs
use crate::core::{UserId, WorldId};
use uuid::Uuid;
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct SessionContext {
    /// Authenticated user, if any
    pub user_id: Option<UserId>,

    /// Currently selected world context
    pub world_id: Option<WorldId>,

    /// Currently selected property context
    pub property_id: Option<Uuid>,

    /// Whether this is an anonymous session
    pub is_anon: bool,
    pub anon_owner_id: Option<Uuid>,
}

impl SessionContext {
    pub fn anonymous(
        engine_user_id: UserId,
        anon_owner_id: Uuid,
    ) -> Self {
        Self {
            user_id: Some(engine_user_id),
            world_id: None,
            property_id: None,
            is_anon: true,
            anon_owner_id: Some(anon_owner_id),
        }
    }

    pub fn authenticated(
        user_id: UserId,
    ) -> Self {
        Self {
            user_id: Some(user_id),
            world_id: None,
            property_id: None,
            is_anon: false,
            anon_owner_id: None,
        }
    }
}
