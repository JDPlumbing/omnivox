use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, sim_duration::SimDuration};
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::world_environment::WorldEnvironment;

use crate::core::physics::illumination::solar_illumination;
use crate::core::physics::insolation::daily_insolation;
use crate::core::physics::tides::tidal_acceleration;
use crate::core::physics::frames::local_tangent_frame;
use crate::core::physics::tides::AnchorError;

#[derive(Debug, Clone, serde::Serialize)]
pub struct EnvironmentalSnapshot {
    pub solar_illumination: f64,
    pub daily_insolation: f64,

    pub lunar_tidal_lateral: f64,
    pub solar_tidal_lateral: f64,
    pub total_tidal_lateral: f64,

    pub density: f64,
    pub pressure: f64,
    pub temperature: f64,
    pub gravity_radial: f64,
}

pub fn sample_environmental_snapshot(
    resolver: &WorldResolver,
    world: WorldId,
    uvox: &UvoxId,
    time: SimTime,
    env: &WorldEnvironment,
) -> Result<EnvironmentalSnapshot, AnchorError> {
    let sun = WorldId(0);

    let illumination = solar_illumination(
        resolver,
        world,
        uvox,
        sun,
        time,
        &env.space,
    )?;

    let insolation = daily_insolation(
        resolver,
        world,
        uvox,
        sun,
        time,
        &env.space,
        144,
    )?;

    let (moon, solar, total) =
        tidal_acceleration(resolver, world, uvox, time, &env.space)?;

    let frame = local_tangent_frame(
        resolver,
        world,
        uvox,
        time,
        &env.space,
    )
    .map_err(|_| AnchorError::Singularity)?;

    let enu = frame.enu;

    let lateral = |v: [f64; 3]| {
        let [e, n, _] = enu.project(v);
        (e * e + n * n).sqrt()
    };

    let env_sample = env.sample(uvox, SimDuration::from_ns(time.0));

    Ok(EnvironmentalSnapshot {
        solar_illumination: illumination,
        daily_insolation: insolation,

        lunar_tidal_lateral: lateral(moon),
        solar_tidal_lateral: lateral(solar),
        total_tidal_lateral: lateral(total),

        density: env_sample.density,
        pressure: env_sample.pressure,
        temperature: env_sample.temperature,
        gravity_radial: env_sample.gravity_radial,
    })
}
