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
}

impl SessionContext {
    /// Anonymous session with no selected context
    pub fn anonymous() -> Self {
        Self {
            user_id: None,
            world_id: None,
            property_id: None,
            is_anon: true,
        }
    }

    /// Authenticated session with no selected world/property yet
    pub fn authenticated(user_id: UserId) -> Self {
        Self {
            user_id: Some(user_id),
            world_id: None,
            property_id: None,
            is_anon: false,
        }
    }
}
