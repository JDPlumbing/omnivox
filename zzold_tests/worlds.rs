use crate::core::worlds::systems::insolation::{compute_insolation};
use crate::core::worlds::systems::frame_mapping::sun_direction_world;
use crate::core::worlds::systems::tidal_potential::compute_tidal_potential; 
use crate::core::physics::units::irradiance::WattsPerSquareMeter;

#[test]
fn insolation_max_at_overhead_sun() {
    let sun_dir = [0.0, 0.0, 1.0]; // straight up
    let normal = [0.0, 0.0, 1.0];
    let stellar_irradiance = WattsPerSquareMeter(1000.0);

    let ins = compute_insolation(sun_dir, stellar_irradiance, normal);

    assert!((ins.flux.0 - 1000.0).abs() < 1e-6);
    assert!((ins.zenith_angle.0).abs() < 1e-6);
}
#[test]
fn insolation_zero_at_night() {
    let sun_dir = [0.0, 0.0, -1.0]; // below horizon
    let normal = [0.0, 0.0, 1.0];
    let stellar_irradiance = WattsPerSquareMeter(1000.0);

    let ins = compute_insolation(sun_dir, stellar_irradiance, normal);
    assert_eq!(ins.flux.0, 0.0);
}
#[test]
fn insolation_at_45_degrees() {
    let sun_dir = crate::core::math::vec3::normalize([1.0, 0.0, 1.0]);
    let normal = [0.0, 0.0, 1.0];
    let stellar_irradiance = WattsPerSquareMeter(1000.0);

    let ins = compute_insolation(sun_dir, stellar_irradiance, normal);
    let expected = 1000.0 * (2.0f64).sqrt() / 2.0;
    assert!((ins.flux.0 - expected).abs() < 1e-6);
}
#[test]
fn sun_direction_maps_to_world_up() {
    let sun_dir_cosmic = [0.0, 0.0, 1.0];
    let north_pole = [0.0, 0.0, 1.0];

    let sun_world = sun_direction_world(sun_dir_cosmic, north_pole);

    assert!((sun_world[2] - 1.0).abs() < 1e-6);
}
#[test]
fn tidal_potential_opposite_on_far_side() {
    let g_center = [0.0, 0.0, -9.8];
    let g_near = [0.0, 0.0, -10.0];
    let g_far  = [0.0, 0.0, -9.6];

    let near = compute_tidal_potential(g_center, g_near);
    let far  = compute_tidal_potential(g_center, g_far);

    assert!(near[2] < 0.0);
    assert!(far[2] > 0.0);
}
