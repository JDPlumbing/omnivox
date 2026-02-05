use crate::core::math::vec3::Vec3;
use crate::core::physics::units::angle::Radians;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;
use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::cosmic::systems::radiation_system::CosmicRadiationSystem;
use crate::core::spatial::surface_coords::SurfaceCoords;
use crate::core::tdt::sim_time::SimTime;
use crate::core::worlds::systems::surface::sample_world_surface;

pub struct Insolation {
    pub flux: WattsPerSquareMeter,
    pub zenith_angle: Radians,
}

pub fn compute_insolation(
    solar_direction: Vec3,
    stellar_irradiance: WattsPerSquareMeter,
    surface_normal: Vec3,
) -> Insolation {
    let s = solar_direction.normalized();
    let n = surface_normal.normalized();

    let cos_theta = s.dot(n).max(0.0);

    let zenith_angle = if cos_theta > 0.0 {
        Radians(cos_theta.acos())
    } else {
        Radians(std::f64::consts::FRAC_PI_2)
    };

    Insolation {
        flux: WattsPerSquareMeter(stellar_irradiance.0 * cos_theta),
        zenith_angle,
    }
}

pub fn insolation_at_surface(
    world_id: WorldId,
    location: &SurfaceCoords,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
    time: SimTime,
) -> Option<Insolation> {
    let anchor = world_state.anchors.get(&world_id)?;
    let body_id = anchor.body;

    let frames = CosmicFrameSystem { state: cosmic_state };
    let radiation_system = CosmicRadiationSystem {
        state: cosmic_state,
        frames: &frames,
    };

    let orbit = cosmic_state.orbits.get(&body_id)?;
    let star_id = orbit.primary;

    let radiation =
        radiation_system.radiation_from_body(star_id, body_id, time)?;

    let body_pose = frames.body_pose(body_id, time);

    // Surface normal (world-local → cosmic)
    let surface_sample =
        sample_world_surface(world_id, location, world_state);

    let surface_normal_cosmic =
        body_pose.orientation * surface_sample.normal_local;

    // Sun direction in cosmic space
    let sun_dir_cosmic = -radiation.direction;

    Some(compute_insolation(
        sun_dir_cosmic,
        radiation.flux,
        surface_normal_cosmic,
    ))
}
