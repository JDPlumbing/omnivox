use crate::core::worlds::id::WorldId;
use crate::core::id::EntityId;
use crate::core::spatial::location::Location;

#[derive(Default)]
pub enum ViewFrame {
    #[default] Cosmic,
    World { world_id: WorldId },
    Environment { world_id: WorldId, location: Location },
    Entity { entity_id: EntityId },
}
