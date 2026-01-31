use crate::shared::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::worlds::id::WorldId;

use crate::core::entity::components::{position::Position,
                                    position_enu::PositionENU,
                                    velocity_enu::VelocityENU,
                                    grounded::Grounded,
                                    world_membership::WorldMembership
                                };
use crate::core::physics::units::{length::Meters,
                                velocity::MetersPerSecond};

pub fn spawn_grounded_entity(
    entities: &mut EntityStore,
    entity: EntityId,
    world_id: WorldId,
    position: Position,
) {
    entities.add_position(entity, position);

    entities.add_position_enu(entity, PositionENU {
        east: Meters(0.0),
        north: Meters(0.0),
        up: Meters(0.0),
    });

    entities.add_velocity_enu(entity, VelocityENU {
        east: MetersPerSecond(0.0),
        north: MetersPerSecond(0.0),
        up: MetersPerSecond(0.0),
    });

    entities.add_grounded(entity, Grounded);
    entities.add_world_membership(entity, WorldMembership { world_id });
    entities.add_active(entity);
}
