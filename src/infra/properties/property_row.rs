// infra/properties/property_row.rs

use uuid::Uuid;
use serde::Deserialize;
use crate::core::{WorldId, UvoxId};

#[derive(Debug, Deserialize)]
pub struct PropertyRow {
    pub property_id: Option<Uuid>,
    pub user_owner_id: Option<Uuid>,
    pub world_id: WorldId,
    pub anchor_uvox: serde_json::Value,
    pub name: Option<String>,
}
// infra/properties/property_row.rs

use crate::core::Property;

impl From<PropertyRow> for Property {
    fn from(row: PropertyRow) -> Self {
        Self {
            id: row
                .property_id
                .expect("property row must have id"),

            owner_user_id: row.user_owner_id,
            world_id: row.world_id,

            anchor: serde_json::from_value(row.anchor_uvox)
                .expect("invalid anchor uvox"),

            name: row.name,
        }
    }
}
