// shared/session/session_context.rs
use crate::core::{UserId, WorldId, SpatialAnchor, SpatialHorizon};
use uuid::Uuid;
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct SessionContext {
    /// Authenticated user (only when logged in)
    pub user_id: Option<UserId>,

    /// Currently selected world context
    pub world_id: Option<WorldId>,

    /// Session-level spatial anchor (pre-entity)
    pub spatial_anchor: Option<SpatialAnchor>,

    pub spatial_horizon: Option<SpatialHorizon>,

    /// Currently selected property context
    pub property_id: Option<Uuid>,

    /// Whether this is an anonymous session
    pub is_anon: bool,

    /// Backing anon identity (only for anon sessions)
    pub anon_owner_id: Option<Uuid>,
}

impl SessionContext {
    /// Pure anonymous session (no auth identity)
    pub fn anonymous(anon_owner_id: Uuid) -> Self {
        Self {
            user_id: None,
            world_id: None,
            spatial_anchor: None,
            spatial_horizon: None,
            property_id: None,
            is_anon: true,
            anon_owner_id: Some(anon_owner_id),
        }
    }

    /// Authenticated session (no spatial/world context yet)
    pub fn authenticated(user_id: UserId) -> Self {
        Self {
            user_id: Some(user_id),
            world_id: None,
            spatial_anchor: None,
            spatial_horizon: None,
            property_id: None,
            is_anon: false,
            anon_owner_id: None,
        }
    }
}

