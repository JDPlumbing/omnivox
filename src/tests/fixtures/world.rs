use crate::core::worlds::state::WorldState;
use crate::core::worlds::id::WorldId;
use crate::core::worlds::components::{world_anchor::WorldAnchor,
                                 world_surface::WorldSurface
                                };
use crate::core::physics::units::length::Meters;
use crate::core::cosmic::id::CosmicBodyId;

pub fn simple_spherical_world(
    world_id: WorldId,
    body: CosmicBodyId,
) -> WorldState {
    let mut world = WorldState::default();

    world.anchors.insert(world_id, WorldAnchor { body });
    world.surfaces.insert(world_id, WorldSurface::Spherical {
        radius: Meters(6_371_000.0),
    });

    world
}
