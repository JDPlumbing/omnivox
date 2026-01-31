use crate::core::math::vec3::Vec3;
use crate::core::physics::units::angle::Radians;
use crate::core::physics::units::irradiance::WattsPerSquareMeter;
use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::cosmic::systems::radiation_system::CosmicRadiationSystem;
use crate::core::spatial::surface::SurfaceCoords;
use crate::core::tdt::sim_time::SimTime;

pub struct Insolation {
    pub flux: WattsPerSquareMeter,
    pub zenith_angle: Radians,
}

pub fn compute_insolation(
    solar_direction_world: Vec3,
    stellar_irradiance: WattsPerSquareMeter,
    surface_normal: Vec3,
) -> Insolation {
    let s = solar_direction_world.normalized();
    let n = surface_normal.normalized();

    let cos_theta = s.dot(n).max(0.0);

    println!("compute_insolation_sun·normal = {}", cos_theta);


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
    _location: &SurfaceCoords, // unused for Option A
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

    // Deterministic star
    let orbit = cosmic_state.orbits.get(&body_id)?;
    let star_id = orbit.primary;

    let radiation =
        radiation_system.radiation_from_body(star_id, body_id, time)?;

    let body_pose = frames.body_pose(body_id, time);

    // Sun direction in planet-fixed frame
    let sun_dir_planet =
        body_pose.orientation.inverse() * (-radiation.direction);

    // Spin axis in planet-fixed frame (Z after orientation)

    let surface_normal = Vec3::new(1.0, 0.0, 0.0);



    let cos_zenith = sun_dir_planet.normalized().dot(surface_normal);



    // Debug
    let rotation = cosmic_state.rotations.get(&body_id).unwrap();
    let t = time.0 as f64 * 1e-9;
    let spin_deg = (2.0 * std::f64::consts::PI * (t / rotation.period.0)
        + rotation.phase_at_epoch.0)
        .rem_euclid(2.0 * std::f64::consts::PI)
        * 180.0 / std::f64::consts::PI;

    println!(
        "INSOLATION_DEBUG spin_deg={:.3} cos_zenith={:.6}",
        spin_deg,
        cos_zenith
    );

    Some(compute_insolation(
        sun_dir_planet,
        radiation.flux,
        surface_normal,
    ))
}
