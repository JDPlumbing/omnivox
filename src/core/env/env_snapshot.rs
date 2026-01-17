use crate::core::env::medium::Medium;
use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::world::world_environment::WorldEnvironment;

pub struct EnvSnapshot {
    pub medium: Medium,
    pub density: f64,
    pub gravity_radial: f64,
    pub pressure: f64,
    pub temperature: f64,
}
pub fn sample_environment(
    env: &WorldEnvironment,
    id: &UvoxId,
    time: SimDuration,
) -> EnvSnapshot {
    let sample = env.sample(id, time);

    EnvSnapshot {
        medium: sample.medium,
        density: sample.density,
        gravity_radial: sample.gravity_radial,
        pressure: sample.pressure,
        temperature: sample.temperature,
    }
}
