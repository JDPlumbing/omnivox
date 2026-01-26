use uuid::Uuid;
use crate::core::{UserId, WorldId};
use crate::core::UvoxId;

pub struct CreateProperty {
    
    pub world_id: WorldId,

    // Domain fields (NOT API DTO)
    pub address_id: Option<Uuid>,
    pub name: Option<String>,
    pub anchor_uvox: UvoxId,

    pub square_feet: Option<i64>,
    pub bedrooms: Option<i64>,
    pub bathrooms: Option<i64>,
    // add more as needed
}
