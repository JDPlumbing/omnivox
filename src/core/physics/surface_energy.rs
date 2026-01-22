use serde::{Serialize, Deserialize};

use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::WorldResolver;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::physics::illumination::solar_illumination;
use crate::core::physics::frames::local_tangent_frame;
use crate::core::math::vec3::{dot, normalize};

/// Solar constant at 1 AU
const SOLAR_CONSTANT_W_M2: f64 = 1361.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SurfaceIrradiance {
    /// Direct sunlight component (W/m²)
    pub direct_w_m2: f64,

    /// Diffuse sky light component (W/m²)
    pub diffuse_w_m2: f64,

    /// Total incoming energy (W/m²)
    pub total_w_m2: f64,
}

pub fn surface_solar_irradiance(
    resolver: &WorldResolver,
    world: WorldId,
    uvox: &UvoxId,
    time: SimTime,
    space: &WorldSpace,
) -> Result<SurfaceIrradiance, EnergyError> {
    // -----------------------------------------
    // Local tangent frame (gives surface normal)
    // -----------------------------------------
    let frame = local_tangent_frame(
        resolver,
        world,
        uvox,
        time,
        space,
    )?;

    // -----------------------------------------
    // Sun direction (world geometry)
    // -----------------------------------------
    let sun_pos = resolver.world_pose(WorldId(0), time).position_m;
    let obs_pos = resolver.world_anchor_point(
        world,
        uvox,
        time,
        space,
    )?;

    let sun_dir = normalize([
        sun_pos[0] - obs_pos[0],
        sun_pos[1] - obs_pos[1],
        sun_pos[2] - obs_pos[2],
    ]);

    // -----------------------------------------
    // Incidence angle
    // -----------------------------------------
    let cos_incidence = dot(frame.enu.up, sun_dir).max(0.0);
        println!("cos_incidence = {}", cos_incidence);

    // Sun below horizon
    if cos_incidence <= 0.0 {
        return Ok(SurfaceIrradiance {
            direct_w_m2: 0.0,
            diffuse_w_m2: 0.0,
            total_w_m2: 0.0,
        });
    }

    // -----------------------------------------
    // Atmospheric attenuation (scalar)
    // -----------------------------------------
    let attenuation = 1.0;
    // -----------------------------------------
    // Energy terms
    // -----------------------------------------
    let direct = SOLAR_CONSTANT_W_M2 * cos_incidence;

    // Diffuse sky light not modeled yet
    let diffuse = 0.0;

    Ok(SurfaceIrradiance {
        direct_w_m2: direct,
        diffuse_w_m2: diffuse,
        total_w_m2: direct + diffuse,
    })
}

#[derive(Debug)]
pub enum EnergyError {
    Frame,
    Geometry,
}

impl From<crate::core::physics::frames::FrameError> for EnergyError {
    fn from(_: crate::core::physics::frames::FrameError) -> Self {
        EnergyError::Frame
    }
}

impl From<crate::core::physics::tides::AnchorError> for EnergyError {
    fn from(_: crate::core::physics::tides::AnchorError) -> Self {
        EnergyError::Geometry
    }
}

impl From<crate::core::world::world_frame::AnchorError> for EnergyError {
    fn from(_: crate::core::world::world_frame::AnchorError) -> Self {
        EnergyError::Geometry
    }
}
