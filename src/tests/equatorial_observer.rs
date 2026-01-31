use crate::core::tdt::SimTime;
use crate::core::spatial::surface::SurfaceCoords;
use crate::core::worlds::systems::insolation::insolation_at_surface;
use crate::core::physics::units::angle::Radians;
use crate::core::physics::units::length::Meters;
use crate::core::physics::units::time::Seconds;
use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::worlds::state::WorldState;
use crate::core::worlds::id::WorldId;
use crate::core::cosmic::components::luminosity::Luminosity;
use crate::core::cosmic::components::orbit::Orbit;
use crate::core::cosmic::components::rotation::Rotation;
use crate::core::cosmic::components::axial_tilt::AxialTilt;
use crate::core::cosmic::components::prime_meridian::PrimeMeridian;
use crate::core::worlds::components::world_anchor::WorldAnchor;
use crate::core::worlds::components::world_surface::WorldSurface;
use crate::core::physics::units::power::Watts;
use crate::core::cosmic::components::root::Root;

const SOLAR_CONSTANT: f64 = 1361.0;
#[test]
fn equatorial_observer_sees_smooth_day_night_cycle() {


    let mut cosmic_state = CosmicState::default();
    let mut world_state = WorldState::default();

    // create star
    let star = CosmicBodyId(0);
    cosmic_state.roots.insert(star, Root);
    cosmic_state.luminosities.insert(star, Luminosity { watts: Watts(3.828e26),});

    // create planet
    let planet = CosmicBodyId(1);
    cosmic_state.orbits.insert(planet, Orbit {
        primary: star,
        semi_major_axis: Meters(1.0e11),
        period: Seconds(365.0 * 86400.0),
        inclination: Radians(0.0),
        phase_at_epoch: Radians(0.0),
    });
    cosmic_state.rotations.insert(planet, Rotation {
        period: Seconds(86400.0),
        phase_at_epoch: Radians(0.0),
    });
    cosmic_state.axial_tilts.insert(planet, AxialTilt {
        radians: Radians(0.0),
    });
    cosmic_state.prime_meridians.insert(planet, PrimeMeridian {
        radians: Radians(0.0),
    });

    // world anchored to planet
    let world_id = WorldId(1);
    world_state.anchors.insert(world_id, WorldAnchor { body: planet });
    world_state.surfaces.insert(
        world_id,
        WorldSurface::Spherical { radius: Meters(6.4e6) },
    );


    let body_id = planet;

    let rotation = cosmic_state.rotations[&planet];

    let steps = 48; // half-hour resolution
    let dt = rotation.period.0 / steps as f64;

    let mut last_flux: Option<f64> = None;
let mut peak_flux: f64 = 0.0;
let mut peak_count: usize = 0;

for i in 0..steps {
    let time_seconds = (i as f64) * dt;
    let time = SimTime::from_seconds(time_seconds as i64);

    let insolation = insolation_at_surface(
        world_id,
        &SurfaceCoords {
            latitude: Radians(0.0),
            longitude: Radians(0.0),
            elevation: Meters(0.0),
        },
        &world_state,
        &cosmic_state,
        time,
    );

    let flux: f64 = insolation.map(|i| i.flux.0).unwrap_or(0.0);

    // 1️⃣ No sudden jumps
    if let Some(prev) = last_flux {
        assert!(
            (flux - prev).abs() < 0.3 * peak_flux.max(1.0_f64),
            "Flux jump detected: {} -> {}",
            prev,
            flux
        );
    }

    // 2️⃣ Track peak
    if flux > peak_flux {
        peak_flux = flux;
        peak_count = 1;
    } else if (flux - peak_flux).abs() < 1e-6 {
        peak_count += 1;
    }

    last_flux = Some(flux);
}

// 3️⃣ Exactly one peak at equator
assert_eq!(
    peak_count, 1,
    "Expected exactly one peak insolation at equator, got {}",
    peak_count
);


    // 4️⃣ Peak must be near solar constant
    assert!(
        peak_flux > 0.95 * SOLAR_CONSTANT,
        "Peak insolation too low: {}",
        peak_flux
    );
}
