use uuid::Uuid;
use crate::core::{UserId, WorldId};
use crate::core::UvoxId;

#[derive(Debug, Clone)]
pub struct UpdateProperty {
    pub property_id: Uuid,
    pub actor_id: UserId,      // who is performing the update
    pub world_id: WorldId,

    pub address_id: Option<Uuid>,
    pub name: Option<String>,
    pub anchor_uvox: Option<UvoxId>,

    pub square_feet: Option<i64>,
    pub bedrooms: Option<i64>,
    pub bathrooms: Option<i64>,
}
