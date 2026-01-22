use crate::core::id::WorldId;
use crate::core::tdt::{SimTime, SimDuration};
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::illumination::solar_illumination;
use crate::core::physics::tides::AnchorError;

/// Integrated solar energy over one planetary rotation (one "day").
///
/// This is a **relative insolation proxy**:
/// - integrates instantaneous solar illumination over time
/// - includes rotation, axial tilt, latitude, orbital geometry
/// - does NOT include atmosphere, albedo, clouds, terrain slope
///
/// Units are arbitrary but **physically proportional**.
///
/// # Parameters
/// - `day_start`: start time of the local day (any reference point)
/// - `samples`: number of time samples over the rotation (e.g. 96–288)
pub fn daily_insolation(
    resolver: &WorldResolver,
    world: WorldId,
    surface: &UvoxId,
    sun: WorldId,
    day_start: SimTime,
    space: &WorldSpace,
    samples: usize,
) -> Result<f64, AnchorError> {
    // ------------------------------------------------------------
    // Determine rotation period
    // ------------------------------------------------------------

    let frame = resolver
        .frames
        .get(&world)
        .expect("missing world frame");

    let rotation_period = match &frame.model {
        crate::core::world::world_frame::FrameModel::Orbital { params } => {
            params.rotation_period
        }
        crate::core::world::world_frame::FrameModel::Static { .. } => {
            return Ok(0.0);
        }
    };

    if rotation_period.is_zero() || samples == 0 {
        return Ok(0.0);
    }

    let dt = rotation_period.0 / samples as i128;
    let dt_seconds = (dt as f64) * 1e-9;

    // ------------------------------------------------------------
    // Integrate illumination over one rotation
    // ------------------------------------------------------------

    let mut total = 0.0;

    for i in 0..samples {
        let t = SimTime(day_start.0 + (i as i128) * dt);

        let flux = solar_illumination(
            resolver,
            world,
            surface,
            sun,
            t,
            space,
        )?;

        total += flux * dt_seconds;
    }

    Ok(total)
}

/// Sample daily insolation over an orbital period (seasonal curve).
///
/// Returns a vector of `(time, daily_insolation)` samples.
pub fn seasonal_insolation_curve(
    resolver: &WorldResolver,
    world: WorldId,
    surface: &UvoxId,
    sun: WorldId,
    year_start: SimTime,
    space: &WorldSpace,
    step: SimDuration,
    day_samples: usize,
) -> Result<Vec<(SimTime, f64)>, AnchorError> {
    let frame = resolver
        .frames
        .get(&world)
        .expect("missing world frame");

    let orbital_period = match &frame.model {
        crate::core::world::world_frame::FrameModel::Orbital { params } => {
            params.period
        }
        _ => return Ok(Vec::new()),
    };

    if orbital_period.is_zero() || step.is_zero() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();
    let mut t = year_start;

    while t.0 < year_start.0 + orbital_period.0 {
        let insolation = daily_insolation(
            resolver,
            world,
            surface,
            sun,
            t,
            space,
            day_samples,
        )?;

        results.push((t, insolation));
        t = SimTime(t.0 + step.0);
    }

    Ok(results)
}
