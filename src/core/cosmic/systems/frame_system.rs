// core/cosmic/systems/frame_system.rs
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::tdt::SimTime;

use crate::core::math::vec3::Vec3;
use crate::core::math::mat3::Mat3;

use super::frame_math::CosmicPose;

pub struct CosmicFrameSystem<'a> {
    pub state: &'a CosmicState,
}

impl<'a> CosmicFrameSystem<'a> {
    pub fn body_pose(
        &self,
        body: CosmicBodyId,
        time: SimTime,
    ) -> CosmicPose {
        self.resolve_pose_recursive(body, time)
    }
}
impl<'a> CosmicFrameSystem<'a> {
    fn resolve_pose_recursive(
        &self,
        body: CosmicBodyId,
        time: SimTime,
    ) -> CosmicPose {
        let local_pose = self.local_body_pose(body, time);

        if let Some(orbit) = self.state.orbits.get(&body) {
            let parent_pose =
                self.resolve_pose_recursive(orbit.primary, time);

            // ✅ Inclination is applied in PARENT equatorial frame
            let inclined =
                Mat3::rotation_x(orbit.inclination.0) * local_pose.position;

            let rotated =
                parent_pose.orientation * inclined;

            CosmicPose {
                position: parent_pose.position + rotated,
                orientation:
                    parent_pose.orientation * local_pose.orientation,
            }
        } else {
            // Root body
            local_pose
        }
    }

}

use std::f64::consts::PI;

impl<'a> CosmicFrameSystem<'a> {
    fn local_body_pose(
    &self,
    body: CosmicBodyId,
    time: SimTime,
) -> CosmicPose {
    // -----------------------------
    // Position (orbit, parent equatorial plane)
    // -----------------------------
    let position = if let Some(orbit) = self.state.orbits.get(&body) {
        let t = time.0 as f64 * 1e-9; // ✅ seconds
        let period = orbit.period.0;

        let theta =
            2.0 * PI * (t / period) + orbit.phase_at_epoch.0;

        let r = orbit.semi_major_axis.0;

        Vec3::new(
            r * theta.cos(),
            r * theta.sin(),
            0.0,
        )
    } else {
        Vec3::new(0.0, 0.0, 0.0)
    };


    // -----------------------------
    // Orientation (prime meridian + axial tilt + spin)
    // -----------------------------
    let align = self
        .state
        .prime_meridians
        .get(&body)
        .map(|p| Mat3::rotation_z(p.radians.0))
        .unwrap_or(Mat3::identity());

    let tilt = self
        .state
        .axial_tilts
        .get(&body)
        .map(|t| Mat3::rotation_x(t.radians.0))
        .unwrap_or(Mat3::identity());

    let spin_angle = self
        .state
        .rotations
        .get(&body)
        .map(|r| {
            let t = time.0 as f64 * 1e-9;
            2.0 * PI * (t / r.period.0) + r.phase_at_epoch.0
        })
        .unwrap_or(0.0);

    let spin_local = Mat3::rotation_z(spin_angle);

    // ✅ Spin about tilted axis
    let spin = tilt * spin_local * tilt.transpose();

    let orientation = spin * align;

    CosmicPose {
        position,
        orientation,
    }
}
}