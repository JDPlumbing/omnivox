
#[test]
fn environment_at_surface_is_physically_sane() {
    // --- Setup ---
    let system = simple_star_planet();
    let world_id = WorldId(1);

    let world = simple_spherical_world(world_id, system.planet);
    let environment = earth_like_environment(world_id);

    let time = SimTime::from_seconds_f64(0.0);

    // Equator, prime meridian, surface
    let surface_coords = SurfaceCoords::on_surface(
        Radians(0.0),
        Radians(0.0),
    );

    // --- World geometry ---
    let surface_sample = sample_world_surface(
        world_id,
        &surface_coords,
        &world,
    );

    // --- Radiation ---
    let radiation = radiation_at_surface(
        world_id,
        &world,
        &system.cosmic,
        time,
    )
    .expect("expected stellar radiation at surface");

    // --- Gravity (vector → scalar) ---
    let gravity_local = gravity_enu_at_location(
        world_id,
        &surface_coords,
        &world,
        &system.cosmic,
        time,
    );

    let gravity_mag = MetersPerSecondSquared(-gravity_local.up.0);

    // --- Environment ---
    let env = environment_at_surface(
        world_id,
        &surface_sample,
        &radiation,
        gravity_mag,
        &environment,
    );

    // --- Assertions ---

    // Insolation must be positive at equator noon
    assert!(
        env.insolation.0 > 1000.0,
        "expected strong insolation at equator noon, got {}",
        env.insolation.0
    );

    // Gravity must be Earth-like
    assert!(
        gravity_mag.0 > 9.0 && gravity_mag.0 < 10.5,
        "unexpected gravity magnitude: {}",
        gravity_mag.0
    );

    // Pressure must be near surface pressure
    assert!(
        env.pressure.0 > 90_000.0,
        "unexpectedly low pressure: {}",
        env.pressure.0
    );

    // Temperature must be finite and non-zero
    assert!(
        env.temperature.0 > 200.0,
        "temperature too low: {}",
        env.temperature.0
    );
}
