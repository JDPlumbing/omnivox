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
    // World 0 = Sun
    // -------------------------------------------------
    frames.insert(
        WorldId(0),
        WorldFrame {
            world_id: WorldId(0),
            parent: None,
            physical_radius_m: Some(696_340_000.0), // Sun radius (m)
            model: FrameModel::Static {
                position: UvoxId::from_vec3([0.0, 0.0, 0.0]),
            },
        },
    );

    // -------------------------------------------------
    // World 1 = Earth
    // -------------------------------------------------
    frames.insert(
        WorldId(1),
        WorldFrame {
            world_id: WorldId(1),
            parent: Some(WorldId(0)),
            physical_radius_m: Some(6_371_000.0), // Earth radius
            model: FrameModel::Orbital {
                params: OrbitalParams {
                    semi_major_axis_m: 149_597_870_700.0,
                    period: SimDuration::years(1),
                    inclination_rad: 0.0,
                    phase_at_epoch: std::f64::consts::PI,

                    rotation_period: SimDuration::seconds(86164),
                    rotation_phase_at_epoch: std::f64::consts::PI,

                    axial_tilt_rad: 0.4090928,
                    prime_meridian_at_epoch: 0.0,

                },
            },
        },
    );

    // -------------------------------------------------
    // World 2 = Moon
    // -------------------------------------------------
    frames.insert(
        WorldId(2),
        WorldFrame {
            world_id: WorldId(2),
            parent: Some(WorldId(1)),
            physical_radius_m: Some(1_737_400.0), // Moon radius
            model: FrameModel::Orbital {
                params: OrbitalParams {
                    semi_major_axis_m: 384_400_000.0,
                    period: SimDuration::days_f64(27.32166),
                    inclination_rad: 0.089,
                    phase_at_epoch: 4.44,
                    rotation_period: SimDuration::days_f64(27.32166),
                    rotation_phase_at_epoch: 0.0,
                    axial_tilt_rad: 0.0269,
                    prime_meridian_at_epoch: 0.0,
                },
            },
        },
    );

        frames
    }
