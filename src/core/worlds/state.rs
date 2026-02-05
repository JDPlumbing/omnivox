// core/worlds/state.rs
use crate::core::math::vec3::Vec3;
use crate::core::physics::units::{length::Meters,};
use std::collections::HashMap;
use crate::core::cosmic::id::CosmicBodyId;

use crate::core::worlds::id::WorldId;
use crate::core::worlds::components::{
        world_anchor::WorldAnchor,
        world_orientation::WorldOrientation,
        world_surface::WorldSurface,
};


#[derive(Default)]
pub struct WorldState {
    pub anchors: HashMap<WorldId, WorldAnchor>,
    pub orientations: HashMap<WorldId, WorldOrientation>,
    pub surfaces: HashMap<WorldId, WorldSurface>,
}

impl WorldState {
    pub fn demo_worlds() -> Self {
        let mut world = Self::default();

        let earth_world = WorldId(1);
        let earth_body  = CosmicBodyId(2); // must match CosmicState

        // Anchor world → cosmic body
        world.anchors.insert(
            earth_world,
            WorldAnchor {
                body: earth_body,
            },
        );

        // World orientation (north = +Z in body frame)
        world.orientations.insert(
            earth_world,
            WorldOrientation {
                north_pole: Vec3::new(0.0, 0.0, 1.0),
            },
        );

        // World surface geometry
        world.surfaces.insert(
            earth_world,
            WorldSurface::Spherical {
                radius: Meters(6_371_000.0),
            },
        );

        world
    }
}
