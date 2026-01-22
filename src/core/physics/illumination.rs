use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::tides::AnchorError;
use crate::core::math::vec3::{dot, normalize, magnitude};

/// Instantaneous solar illumination at a surface point
pub fn solar_illumination(
    resolver: &WorldResolver,
    world: WorldId,
    surface: &UvoxId,
    sun: WorldId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<f64, AnchorError> {
    let surface_pos =
        resolver.world_anchor_point(world, surface, time, space)?;

    let sun_pos =
        resolver.world_pose(sun, time).position_m;

    let to_sun = [
        sun_pos[0] - surface_pos[0],
        sun_pos[1] - surface_pos[1],
        sun_pos[2] - surface_pos[2],
    ];

    let dist = magnitude(to_sun).max(1.0);
    let sun_dir = normalize(to_sun);

    let pose = resolver.world_pose(world, time);
    let world_normal = pose.orientation * local_surface_normal(surface)?;

    let cos_incidence = dot(world_normal, sun_dir).max(0.0);

    Ok(cos_incidence / (dist * dist))
}

/// ------------------------------------------------------------
/// Geometry helpers (local, canonical for illumination)
/// ------------------------------------------------------------

fn local_surface_normal(surface: &UvoxId) -> Result<[f64; 3], AnchorError> {
    if surface.is_origin() {
        return Err(AnchorError::Singularity);
    }

    let v = surface.to_vec3();

    let x = v[0] as f64;
    let y = v[1] as f64;
    let z = v[2] as f64;

    let mag = (x * x + y * y + z * z).sqrt().max(1.0);

    Ok([x / mag, y / mag, z / mag])
}




/// World-space (inertial) unit vector pointing FROM surface anchor TO the sun
pub fn sun_direction_world(
    resolver: &WorldResolver,
    world: WorldId,
    surface: &UvoxId,
    sun: WorldId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<[f64; 3], AnchorError> {

    // Surface position (already includes rotation via anchor)
    let surface_pos =
        resolver.world_anchor_point(world, surface, time, space)?;

    // Sun position (pure inertial)
    let sun_pos =
        resolver.world_pose(sun, time).position_m;

    // Inertial direction vector
    let to_sun_inertial = [
        sun_pos[0] - surface_pos[0],
        sun_pos[1] - surface_pos[1],
        sun_pos[2] - surface_pos[2],
    ];

    Ok(normalize(to_sun_inertial))
}
