// core/physics/lunar_phase.rs

use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::world::world_frame::WorldResolver;
use crate::core::UvoxId;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::AnchorError;
use crate::core::math::vec3::{dot, normalize, magnitude};


use serde::{Serialize};
/// Result of lunar phase computation
#[derive(Debug, Clone, Copy, Serialize)]
pub struct LunarPhase {
    /// Phase angle in radians (0 = new, π = full)
    pub phase_angle_rad: f64,

    /// Fraction illuminated [0.0, 1.0]
    pub illuminated_fraction: f64,
}

/// Compute the Moon’s phase using pure world geometry
///
/// Uses only world origins:
/// - Moon → Sun
/// - Moon → Earth
pub fn lunar_phase(
    resolver: &WorldResolver,
    moon: WorldId,
    earth: WorldId,
    sun: WorldId,
    time: SimTime,
) -> LunarPhase {
    let moon_pos = resolver.world_pose(moon, time).position_m;
    let earth_pos = resolver.world_pose(earth, time).position_m;
    let sun_pos = resolver.world_pose(sun, time).position_m;

    let moon_to_sun = [
        sun_pos[0] - moon_pos[0],
        sun_pos[1] - moon_pos[1],
        sun_pos[2] - moon_pos[2],
    ];

    let moon_to_earth = [
        earth_pos[0] - moon_pos[0],
        earth_pos[1] - moon_pos[1],
        earth_pos[2] - moon_pos[2],
    ];

    let dot = dot(moon_to_sun, moon_to_earth);
    let mag = magnitude(moon_to_sun) * magnitude(moon_to_earth);

    let cos_phi = (dot / mag).clamp(-1.0, 1.0);
    let phase_angle = cos_phi.acos();

    let illuminated_fraction = 0.5 * (1.0 + cos_phi);

    LunarPhase {
        phase_angle_rad: phase_angle,
        illuminated_fraction,
    }
}


pub fn observer_lunar_phase_fraction(
    resolver: &WorldResolver,
    earth: WorldId,
    moon: WorldId,
    sun: WorldId,
    observer_uvox: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<f64, AnchorError> {
    // Positions
    let moon_pos = resolver.world_pose(moon, time).position_m;
    let sun_pos  = resolver.world_pose(sun, time).position_m;

    let obs_pos = resolver.world_anchor_point(
        earth,
        observer_uvox,
        time,
        space,
    )?;

    // Direction vectors (from Moon)
    let to_sun = normalize([
        sun_pos[0] - moon_pos[0],
        sun_pos[1] - moon_pos[1],
        sun_pos[2] - moon_pos[2],
    ]);

    let to_obs = normalize([
        obs_pos[0] - moon_pos[0],
        obs_pos[1] - moon_pos[1],
        obs_pos[2] - moon_pos[2],
    ]);

    let cos_phase = dot(to_sun, to_obs).clamp(-1.0, 1.0);

    Ok((1.0 + cos_phase) * 0.5)
}
