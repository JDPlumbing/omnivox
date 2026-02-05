use crate::core::math::vec3::Vec3;
use crate::core::spatial::surface_coords::SurfaceCoords;
use crate::core::worlds::components::world_surface::WorldSurface;
use crate::core::worlds::state::WorldState;
use crate::core::worlds::systems::geometry::surface_normal_from_lat_lon;
use crate::core::worlds::id::WorldId;
use crate::core::physics::units::length::Meters;
use crate::core::physics::units::angle::Radians;

/// Pure geometric sample of a world's surface at a given location.
/// Contains no physics, no environment, and no time-dependent state.
#[derive(Debug, Clone, Copy)]
pub struct WorldSurfaceSample {
    /// Elevation relative to the world's reference surface.
    pub height: Meters,

    /// Surface normal in the world-body local frame.
    pub normal_local: Vec3,
}

/// Sample geometric properties of a world surface at the given coordinates.
///
/// This function is purely geometric:
/// - no cosmic state
/// - no environment state
/// - no physics side effects
///
/// It defines the canonical surface orientation used by
/// insolation, gravity projection, and environment sampling.
pub fn sample_world_surface(
    world_id: WorldId,
    location: &SurfaceCoords,
    world_state: &WorldState,
) -> WorldSurfaceSample {
    let surface = world_state
        .surfaces
        .get(&world_id)
        .expect("world has no surface");

    match surface {
        WorldSurface::Spherical { .. } => {
            // 🌍 WORLD → GEOMETRY boundary
            let lat_rad = Radians::from_degrees(location.latitude);
            let lon_rad = Radians::from_degrees(location.longitude);

            let normal = surface_normal_from_lat_lon(
                lat_rad,
                lon_rad,
            );

            WorldSurfaceSample {
                height: location.elevation,
                normal_local: normal,
            }
        }

        WorldSurface::Plane { elevation } => {
            WorldSurfaceSample {
                height: *elevation,
                normal_local: Vec3::new(0.0, 0.0, 1.0),
            }
        }
    }
}
