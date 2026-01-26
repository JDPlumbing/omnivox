use uuid::Uuid;
use crate::core::{WorldId, UvoxId};
use serde::Serialize;
// core/property/property.rs
use crate::supabasic::properties::PropertyRecord;

#[derive(Debug, Clone, Serialize)]
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




impl From<PropertyRecord> for Property {
    fn from(record: PropertyRecord) -> Self {
        Self {
            id: record
                .property_id
                .expect("property record must have id"),

            owner_user_id: record.user_owner_id,

            world_id: record.world_id,

            anchor: serde_json::from_value(record.anchor_uvox)
                .expect("invalid anchor uvox"),

            name: record.name,
        }
    }
}
