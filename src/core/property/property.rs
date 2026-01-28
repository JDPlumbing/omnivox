// core/property/property.rs

use uuid::Uuid;
use crate::core::{WorldId, UvoxId};

#[derive(Debug, Clone)]
pub struct Property {
    pub id: Uuid,

    // Ownership / association
    pub owner_user_id: Option<Uuid>,

    // Spatial anchoring
    pub world_id: WorldId,
    pub anchor: UvoxId,

    // Human-facing metadata
    pub name: Option<String>,
}
