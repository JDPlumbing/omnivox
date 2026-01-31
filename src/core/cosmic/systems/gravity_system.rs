// core/cosmic/systems/gravity_system.rs

use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::tdt::SimTime;

use crate::core::math::vec3::Vec3;
use crate::core::physics::units::acceleration::MetersPerSecondSquared;

use super::gravity_math::GravitationalAcceleration;

pub struct CosmicGravitySystem<'a> {
    pub state: &'a CosmicState,
    pub frames: &'a CosmicFrameSystem<'a>,
}
impl<'a> CosmicGravitySystem<'a> {
    pub fn acceleration_from_body(
        &self,
        target: CosmicBodyId,
        source: CosmicBodyId,
        time: SimTime,
    ) -> Option<GravitationalAcceleration> {
        let target_pose = self.frames.body_pose(target, time);
        let source_pose = self.frames.body_pose(source, time);

        let source_mass = self.state.masses.get(&source)?;

        // Vector from target → source
        let r_vec: Vec3 = source_pose.position - target_pose.position;

        let r = r_vec.magnitude().max(1.0);
        let direction = r_vec.normalized();

        let a_mag = crate::core::physics::constants::universal::G
            * source_mass.kg.0
            / (r * r);

        Some(GravitationalAcceleration {
            direction,
            magnitude: MetersPerSecondSquared(a_mag),
        })
    }
}
impl<'a> CosmicGravitySystem<'a> {
    pub fn total_acceleration(
        &self,
        target: CosmicBodyId,
        time: SimTime,
    ) -> GravitationalAcceleration {
        let mut total_vec = Vec3::new(0.0, 0.0, 0.0);

        for (&other, _) in &self.state.masses {
            if other == target {
                continue;
            }

            if let Some(a) =
                self.acceleration_from_body(target, other, time)
            {
                total_vec =
                    total_vec + a.direction * a.magnitude.0;
            }
        }

        let magnitude = total_vec.magnitude();

        if magnitude > 0.0 {
            GravitationalAcceleration {
                direction: total_vec.normalized(),
                magnitude: MetersPerSecondSquared(magnitude),
            }
        } else {
            GravitationalAcceleration {
                direction: Vec3::new(0.0, 0.0, 0.0),
                magnitude: MetersPerSecondSquared(0.0),
            }
        }
    }
}
