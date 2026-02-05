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
            orientation: parent_pose.orientation * local_pose.orientation,
        }
       
        } else {
            // Root body
            local_pose
        }
    }
}

#[allow(non_snake_case)]
fn solve_kepler(M: f64, e: f64) -> f64 {
    let mut E = M; // good initial guess

    for _ in 0..5 {
        E = E - (E - e * E.sin() - M) / (1.0 - e * E.cos());
    }

    E
}

use std::f64::consts::PI;

impl<'a> CosmicFrameSystem<'a> {
    #[allow(non_snake_case)]
    fn local_body_pose(
        &self,
        body: CosmicBodyId,
        time: SimTime,
    ) -> CosmicPose {
    // -----------------------------
    // Orbit solution
    // -----------------------------
    let (position, orbit_phase) = if let Some(orbit) = self.state.orbits.get(&body) {
        let t = time.0 as f64 * 1e-9;
        let period = orbit.period.0;

        let a = orbit.semi_major_axis.0;
        let e = orbit.eccentricity;

        // Mean anomaly
        let M = 2.0 * PI * (t / period) + orbit.phase_at_epoch.0;

        // Eccentric anomaly
        let E = solve_kepler(M, e);

        // True anomaly
        let sin_v = ((1.0 - e * e).sqrt() * E.sin()) / (1.0 - e * E.cos());
        let cos_v = (E.cos() - e) / (1.0 - e * E.cos());
        let theta = sin_v.atan2(cos_v);
       /* if body == CosmicBodyId(2) {
            println!(
                "t_days={:.2}  Earth true anomaly={:.2}°",
                t / 86400.0,
                theta.to_degrees()
            );
        }*/

        // Radius
        let r = a * (1.0 - e * e) / (1.0 + e * theta.cos());

        let position = Vec3::new(
            r * theta.cos(),
            r * theta.sin(),
            0.0,
        );

        (position, Mat3::rotation_z(theta))
    } else {
        (Vec3::ZERO, Mat3::identity())
    };

    // -----------------------------
    // Orientation (orbit phase + axial tilt + spin)
    // -----------------------------
    // Prime meridian
    let align = self
        .state
        .prime_meridians
        .get(&body)
        .map(|p| Mat3::rotation_z(p.radians.0))
        .unwrap_or(Mat3::identity());

    // Axial tilt params
    let (tilt_angle, tilt_longitude) = self
        .state
        .axial_tilts
        .get(&body)
        .map(|t| (t.radians.0, t.longitude.0))
        .unwrap_or((0.0, 0.0));

    // Tilt direction + magnitude
    let tilt_dir = Mat3::rotation_z(tilt_longitude);
    let tilt_mag = Mat3::rotation_x(tilt_angle);

    // Fixed spin axis in inertial space (THIS is what creates seasons)
    let tilt_frame = tilt_dir * tilt_mag;


    // Spin
    let spin_angle = self
        .state
        .rotations
        .get(&body)
        .map(|r| {
            let t = time.0 as f64 * 1e-9;
            2.0 * PI * (t / r.period.0) + r.phase_at_epoch.0
        })
        .unwrap_or(0.0);

    // Spin about tilted axis
    let spin_about_axis =
        tilt_frame * Mat3::rotation_z(spin_angle) * tilt_frame.transpose();

    // Final orientation
    let orientation = spin_about_axis * align;


    CosmicPose {
        position,
        orientation,
    }
}
}