use crate::core::tdt::SimTime;
use crate::core::cosmic::systems::frame_system::CosmicFrameSystem;
use crate::core::worlds::systems::geometry::surface_normal_from_lat_lon;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::cosmic::components::{
    root::Root,
    orbit::Orbit,
    rotation::Rotation,
    axial_tilt::AxialTilt,
    prime_meridian::PrimeMeridian,
    radius::Radius,
};
use crate::core::physics::units::{
    length::Meters,
    time::Seconds,
    angle::Radians,
};

fn setup_simple_star_planet() -> (CosmicState, CosmicBodyId, CosmicBodyId) {
    let mut cosmic = CosmicState::default();

    let star   = CosmicBodyId(1);
    let planet = CosmicBodyId(2);

    // 🌞 Star is a root (no orbit)
    cosmic.roots.insert(star, Root {});
    cosmic.radii.insert(
        star,
            Radius {
            meters: Meters(1.0),
        },
    );

    // 🌍 Planet orbits star in equatorial plane
    cosmic.radii.insert(
        planet,
        Radius {
            meters: Meters(6_371_000.0),
        },
    );  
    cosmic.orbits.insert(
        planet,
        Orbit {
            primary: star,
            semi_major_axis: Meters(1.0),
            period: Seconds(365.0 * 86_400.0), // doesn't matter for geometry
            inclination: Radians(0.0),
            phase_at_epoch: Radians(0.0),
        },
    );

    // 🌀 Planet rotation (24h)
    cosmic.rotations.insert(
        planet,
        Rotation {
            period: Seconds(86_400.0),
            phase_at_epoch: Radians(0.0),
        },
    );

    // 🌍 No axial tilt (equatorial sanity test)
    cosmic.axial_tilts.insert(
        planet,
        AxialTilt {
            radians: Radians(0.0),
        },
    );

    // 📍 Prime meridian aligned at epoch
    cosmic.prime_meridians.insert(
        planet,
        PrimeMeridian {
            radians: Radians(0.0),
        },
    );

    (cosmic, star, planet)
}



#[test]
fn equatorial_observer_sees_smooth_solar_arc() {
    let (cosmic_state, star, planet) = setup_simple_star_planet();

    let frames = CosmicFrameSystem { state: &cosmic_state };

    let surface_normal = surface_normal_from_lat_lon(
        Radians(0.0),
        Radians(0.0),
    );

    let rotation = cosmic_state.rotations[&planet];
    let steps = 48;
    let dt = rotation.period.0 / steps as f64;

    let mut last: Option<f64> = None;

    let mut peak_count = 0;
    let mut peak: f64 = -1.0;


    for i in 0..steps {
        let time = SimTime::from_seconds_f64(i as f64 * dt);



        let star_pose   = frames.body_pose(star, time);
        let planet_pose = frames.body_pose(planet, time);

        let sun_dir_cosmic =
            (star_pose.position - planet_pose.position).normalized();
        
        let sun_dir_planet =
            planet_pose.orientation.inverse() * sun_dir_cosmic;

        let cos_zenith: f64 = sun_dir_planet.dot(surface_normal);


        // 🔒 continuity
        if let Some(prev) = last {
            let delta = (cos_zenith - prev).abs();

            assert!(
                delta < 0.5, // see note below
                "cos_zenith jump: {} → {} (Δ={})",
                prev, cos_zenith, delta
            );
        }


        // 🔺 peak tracking
        if cos_zenith > peak {
            peak = cos_zenith;
            peak_count = 1;
        } else if (cos_zenith - peak).abs() < 1e-6 {
            peak_count += 1;
        }

        last = Some(cos_zenith);
    }

    assert_eq!(peak_count, 1, "Sun should peak exactly once");
}

#[test]
fn axial_tilt_reduces_noon_sun_at_mid_latitude() {
    let (mut cosmic_state, star, planet) = setup_simple_star_planet();

    // 🌍 Earth-like axial tilt
    cosmic_state.axial_tilts.insert(
        planet,
        AxialTilt {
            radians: Radians(23.44_f64.to_radians()),
        },
    );

    let frames = CosmicFrameSystem { state: &cosmic_state };

    // 📍 Observer at 45° latitude
    let surface_normal = surface_normal_from_lat_lon(
        Radians(45.0_f64.to_radians()),
        Radians(0.0),
    );

    let rotation = cosmic_state.rotations[&planet];
    let steps = 96; // finer sampling
    let dt = rotation.period.0 / steps as f64;

    let mut last: Option<f64> = None;
    let mut peak = -1.0;
    let mut peak_count = 0;

    for i in 0..steps {
        let time = SimTime::from_seconds_f64(i as f64 * dt);

        let star_pose   = frames.body_pose(star, time);
        let planet_pose = frames.body_pose(planet, time);

        let sun_dir_cosmic =
            (star_pose.position - planet_pose.position).normalized();

        let sun_dir_planet =
            planet_pose.orientation.inverse() * sun_dir_cosmic;

        let cos_zenith = sun_dir_planet.dot(surface_normal);

        // 🔒 continuity
        if let Some(prev) = last {
            let delta = (cos_zenith - prev).abs();
            assert!(
                delta < 0.5,
                "cos_zenith jump: {} → {} (Δ={})",
                prev, cos_zenith, delta
            );
        }

        // 🔺 peak tracking
        if cos_zenith > peak {
            peak = cos_zenith;
            peak_count = 1;
        } else if (cos_zenith - peak).abs() < 1e-6 {
            peak_count += 1;
        }

        last = Some(cos_zenith);
    }

    // ☀️ Sun should peak exactly once
    assert_eq!(peak_count, 1, "Sun should peak exactly once");

    // 🌍 With tilt + latitude, Sun should NOT reach zenith
    assert!(
        peak < 0.9,
        "Expected reduced noon Sun due to axial tilt, got cos_zenith={}",
        peak
    );
}


#[test]
fn axial_tilt_produces_seasonal_noon_variation() {
    let (mut cosmic_state, star, planet) = setup_simple_star_planet();

    // 🌍 Earth-like axial tilt
    cosmic_state.axial_tilts.insert(
        planet,
        AxialTilt {
            radians: Radians(23.44_f64.to_radians()),
        },
    );

    // 🕒 Slow orbit so rotation dominates day
    cosmic_state.orbits.get_mut(&planet).unwrap().period =
        Seconds(365.0 * 86_400.0);

    let frames = CosmicFrameSystem { state: &cosmic_state };

    // 📍 Observer at 45° latitude
    let surface_normal = surface_normal_from_lat_lon(
        Radians(45.0_f64.to_radians()),
        Radians(0.0),
    );

    // 🌞 Sample four orbital phases (quarter year steps)
    let orbit_period = cosmic_state.orbits[&planet].period.0;

    let orbital_phases = [
        0.0,
        0.25 * orbit_period,
        0.50 * orbit_period,
        0.75 * orbit_period,
    ];

    let mut noon_values = Vec::new();

    for &t_orbit in &orbital_phases {
        // Noon = rotation phase 0
        let time = SimTime::from_seconds_f64(t_orbit);

        let star_pose   = frames.body_pose(star, time);
        let planet_pose = frames.body_pose(planet, time);

        let sun_dir_cosmic =
            (star_pose.position - planet_pose.position).normalized();

        let sun_dir_planet =
            planet_pose.orientation.inverse() * sun_dir_cosmic;

        let cos_zenith = sun_dir_planet.dot(surface_normal);

        noon_values.push(cos_zenith);
    }

    let equinox_1 = noon_values[0];
    let solstice_1 = noon_values[1];
    let equinox_2 = noon_values[2];
    let solstice_2 = noon_values[3];

    // 🌗 Equinoxes should match
    assert!(
        (equinox_1 - equinox_2).abs() < 1e-6,
        "Equinox noon Sun heights should match: {} vs {}",
        equinox_1,
        equinox_2
    );

    // ☀️ Solstices should differ
    assert!(
        (solstice_1 - solstice_2).abs() > 0.05,
        "Solstice noon Sun heights should differ: {} vs {}",
        solstice_1,
        solstice_2
    );

    // 🔺 One solstice must be higher (summer vs winter)
    let max_solstice = solstice_1.max(solstice_2);
    let min_solstice = solstice_1.min(solstice_2);

    assert!(
        max_solstice > equinox_1,
        "Summer solstice should exceed equinox noon Sun"
    );

    assert!(
        min_solstice < equinox_1,
        "Winter solstice should be below equinox noon Sun"
    );
}
