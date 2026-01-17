// core/world/presets/frames.rs

use std::collections::HashMap;

use crate::core::id::WorldId;
use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_frame::{
    WorldFrame, FrameModel, OrbitalParams,
};

/// Hardcoded frame presets for now
/// This is NOT environment
/// This is spatial hierarchy
pub fn frame_presets() -> HashMap<WorldId, WorldFrame> {
    let mut frames = HashMap::new();

    // -------------------------------------------------
    // World 0 = Sun (root frame)
    // -------------------------------------------------
    frames.insert(
        WorldId(0),
        WorldFrame {
            world_id: WorldId(0),
            parent: None,
            model: FrameModel::Static {
                position: UvoxId::from_vec3([0.0, 0.0, 0.0]),
            },
        },
    );

    // -------------------------------------------------
    // World 1 = Earth (orbits Sun)
    // -------------------------------------------------
    frames.insert(
        WorldId(1),
        WorldFrame {
            world_id: WorldId(1),
            parent: Some(WorldId(0)),
            model: FrameModel::Orbital {
            params: OrbitalParams {
            semi_major_axis_m: 149_597_870_700.0, // meters
            period: SimDuration::years(1),      // or SimDuration::years(1)

                inclination_rad: 0.0,
                phase_at_epoch: 0.0,

                rotation_period: SimDuration::seconds(86164), // sidereal day
                rotation_phase_at_epoch: 0.0,
                axial_tilt_rad: 0.4090928, // 23.439281°
            }

            },
        },
    );

    // -------------------------------------------------
    // World 2 = Moon (orbits Earth)
    // -------------------------------------------------
    frames.insert(
        WorldId(2),
        WorldFrame {
            world_id: WorldId(2),
            parent: Some(WorldId(1)),
            model: FrameModel::Orbital {
                params: OrbitalParams {
                    semi_major_axis_m: 384_400_000.0,
                    period: SimDuration::days(27),
                    inclination_rad: 0.089,
                    phase_at_epoch: 4.71238898, // Last Quarter Moon


                    // Moon rotation (tidally locked, approx)
                    rotation_period: SimDuration::days(27),
                    rotation_phase_at_epoch: 0.0,
                    axial_tilt_rad: 0.0269, // ~1.54°
                },

            },
        },
    );

    frames
}
