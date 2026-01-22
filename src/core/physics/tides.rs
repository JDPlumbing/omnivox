use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::world::world_frame::{self, WorldResolver};

/// ------------------------------------------------------------
/// Errors
/// ------------------------------------------------------------
#[derive(Debug)]
pub enum AnchorError {
    Singularity,
    World(world_frame::AnchorError),
}

impl From<world_frame::AnchorError> for AnchorError {
    fn from(err: world_frame::AnchorError) -> Self {
        AnchorError::World(err)
    }
}

/// ------------------------------------------------------------
/// Constants
/// ------------------------------------------------------------

/// Gravitational constant (m³ / kg / s²)
const G: f64 = 6.67430e-11;

/// Approximate masses (kg)
const MASS_SUN: f64  = 1.98847e30;
const MASS_MOON: f64 = 7.342e22;

/// ------------------------------------------------------------
/// Public API
/// ------------------------------------------------------------

/// Full tidal potential (Moon, Sun, total)
pub fn tidal_potential(
    resolver: &WorldResolver,
    world: WorldId,
    anchor: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<(f64, f64, f64), AnchorError> {
    let lunar = tidal_potential_body(
        resolver,
        world,
        anchor,
        WorldId(2),
        MASS_MOON,
        time,
        space,
    )?;

    let solar = tidal_potential_body(
        resolver,
        world,
        anchor,
        WorldId(0),
        MASS_SUN,
        time,
        space,
    )?;

    Ok((lunar, solar, lunar + solar))
}

/// World-space tidal acceleration vectors (Moon, Sun, total)
pub fn tidal_acceleration(
    resolver: &WorldResolver,
    world: WorldId,
    anchor: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<([f64; 3], [f64; 3], [f64; 3]), AnchorError> {
    let moon = tidal_acceleration_body(
        resolver, world, anchor, WorldId(2), MASS_MOON, time, space,
    )?;

    let sun = tidal_acceleration_body(
        resolver, world, anchor, WorldId(0), MASS_SUN, time, space,
    )?;

    let total = [
        moon[0] + sun[0],
        moon[1] + sun[1],
        moon[2] + sun[2],
    ];

    Ok((moon, sun, total))
}

/// Tangential (surface-parallel) lunar tidal acceleration
pub fn lunar_tidal_acceleration_tangent(
    resolver: &WorldResolver,
    world: WorldId,
    anchor: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<[f64; 3], AnchorError> {
    let a_world = tidal_acceleration_body(
        resolver,
        world,
        anchor,
        WorldId(2),
        MASS_MOON,
        time,
        space,
    )?;

    let pose = resolver.world_pose(world, time);
    let n_world = pose.orientation * local_surface_normal(anchor)?;

    Ok(project_to_tangent(a_world, n_world))
}

/// ------------------------------------------------------------
/// Core physics
/// ------------------------------------------------------------

fn tidal_potential_body(
    resolver: &WorldResolver,
    surface_world: WorldId,
    anchor: &UvoxId,
    body_world: WorldId,
    body_mass: f64,
    time: SimTime,
    space: &WorldSpace,
) -> Result<f64, AnchorError> {
    let surface_pos =
        resolver.world_anchor_point(surface_world, anchor, time, space)?;

    let body_pos = resolver.world_pose(body_world, time).position_m;

    let r_vec = [
        body_pos[0] - surface_pos[0],
        body_pos[1] - surface_pos[1],
        body_pos[2] - surface_pos[2],
    ];

    let r = magnitude(r_vec).max(1.0);
    let r_hat = normalize(r_vec);

    let pose = resolver.world_pose(surface_world, time);
    let n_hat = pose.orientation * local_surface_normal(anchor)?;

    let cos_theta = dot(n_hat, r_hat);

    Ok(
        (G * body_mass / (r * r * r))
            * 0.5
            * (3.0 * cos_theta * cos_theta - 1.0),
    )
}

fn tidal_acceleration_body(
    resolver: &WorldResolver,
    surface_world: WorldId,
    anchor: &UvoxId,
    body_world: WorldId,
    body_mass: f64,
    time: SimTime,
    space: &WorldSpace,
) -> Result<[f64; 3], AnchorError> {
    let surface_pos =
        resolver.world_anchor_point(surface_world, anchor, time, space)?;

    let body_pos = resolver.world_pose(body_world, time).position_m;

    let r_vec = [
        body_pos[0] - surface_pos[0],
        body_pos[1] - surface_pos[1],
        body_pos[2] - surface_pos[2],
    ];

    let r = magnitude(r_vec).max(1.0);
    let r_hat = normalize(r_vec);

    let pose = resolver.world_pose(surface_world, time);
    let n_hat = pose.orientation * local_surface_normal(anchor)?;

    let cos_theta = dot(n_hat, r_hat);
    let factor = G * body_mass / (r * r * r);

    Ok([
        factor * (3.0 * cos_theta * r_hat[0] - n_hat[0]),
        factor * (3.0 * cos_theta * r_hat[1] - n_hat[1]),
        factor * (3.0 * cos_theta * r_hat[2] - n_hat[2]),
    ])
}

/// ------------------------------------------------------------
/// Geometry helpers
/// ------------------------------------------------------------

fn local_surface_normal(anchor: &UvoxId) -> Result<[f64; 3], AnchorError> {
    if anchor.is_origin() {
        return Err(AnchorError::Singularity);
    }

    let v = anchor.to_vec3();

    let x = v[0] as f64;
    let y = v[1] as f64;
    let z = v[2] as f64;

    let mag = (x * x + y * y + z * z).sqrt().max(1.0);

    Ok([x / mag, y / mag, z / mag])
}

fn project_to_tangent(
    accel_world: [f64; 3],
    normal_world: [f64; 3],
) -> [f64; 3] {
    let dot_an = dot(accel_world, normal_world);

    [
        accel_world[0] - dot_an * normal_world[0],
        accel_world[1] - dot_an * normal_world[1],
        accel_world[2] - dot_an * normal_world[2],
    ]
}

/// ------------------------------------------------------------
/// Math helpers (canonical for tides)
/// ------------------------------------------------------------

pub fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

pub fn normalize(v: [f64; 3]) -> [f64; 3] {
    let m = magnitude(v).max(1e-12);
    [v[0] / m, v[1] / m, v[2] / m]
}
