use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::tdt::SimTime;

use crate::core::math::vec3::Vec3;
use crate::core::physics::units::length::Meters;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;

use super::radiation_math::RadiationSample;


pub struct CosmicRadiationSystem<'a> {
    pub state: &'a CosmicState,
    pub frames: &'a CosmicFrameSystem<'a>,
}
impl<'a> CosmicRadiationSystem<'a> {
    pub fn radiation_from_body(
        &self,
        source: CosmicBodyId,
        target: CosmicBodyId,
        time: SimTime,
    ) -> Option<RadiationSample> {
        let luminosity = self.state.luminosities.get(&source)?;

        let source_pose = self.frames.body_pose(source, time);
        let target_pose = self.frames.body_pose(target, time);

        // Vector from target → source
        let vec: Vec3 = source_pose.position - target_pose.position;

        let distance_m = vec.magnitude().max(1.0);
        let direction = vec.normalized();

        let flux = luminosity.watts.0
            / (4.0 * std::f64::consts::PI * distance_m * distance_m);

        Some(RadiationSample {
            direction,
            distance: Meters(distance_m),
            flux: WattsPerSquareMeter(flux),
        })
    }


pub fn total_radiation_at_body(
    &self,
    target: CosmicBodyId,
    time: SimTime,
) -> Vec<(CosmicBodyId, RadiationSample)> {
    let mut out = Vec::new();

    for (&source, _) in &self.state.luminosities {
        if source == target {
            continue;
        }

        if let Some(sample) =
            self.radiation_from_body(source, target, time)
        {
            out.push((source, sample));
        }
    }

    out
}

}