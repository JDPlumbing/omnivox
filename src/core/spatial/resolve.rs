use crate::core::spatial::uvox_id::{UvoxId, RUm, LatCode, LonCode};
use crate::core::spatial::surface_coords::SurfaceCoords;

use crate::core::worlds::id::WorldId;
use crate::core::worlds::state::WorldState;

use crate::core::cosmic::state::CosmicState;
use crate::core::physics::units::angle::Degrees;
use crate::core::physics::units::length::Meters;
use crate::core::math::vec3::Vec3;


/// Angular scale: integer units per degree
/// Chosen to give ~1.1µm resolution at Earth's equator.
pub const ANG_SCALE: i128 = 100_000_000_000;

/// Convert degrees → integer angular code
fn degrees_to_code(deg: Degrees) -> i64 {
    (deg.0 * ANG_SCALE as f64).round() as i64
}

/// Convert integer angular code → degrees
fn code_to_degrees(code: i64) -> Degrees {
    Degrees(code as f64 / ANG_SCALE as f64)
}

/// Resolve a world-surface position into a spatial address.
///
/// This is the ONLY place UvoxId is constructed.
pub fn surface_to_uvox(
    world_id: WorldId,
    surface: SurfaceCoords,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
) -> UvoxId {
    // 1️⃣ Resolve world anchor → cosmic body
    let anchor = world_state
        .anchors
        .get(&world_id)
        .expect("world has no anchor");

    let body_id = anchor.body;

    // 2️⃣ Get cosmic body reference radius (meters)
    let base_radius_m = cosmic_state
        .radii
        .get(&body_id)
        .expect("cosmic body has no radius")
        .meters
        .0;

    // 3️⃣ Compute absolute radius (micrometers)
    let r_um = ((base_radius_m + surface.elevation.0) * 1_000_000.0)
        .round() as i64;

    // 4️⃣ Encode angular coordinates (degrees → scaled integers)
    let lat_code = LatCode(degrees_to_code(surface.latitude));
    let lon_code = LonCode(degrees_to_code(surface.longitude));

    // 5️⃣ Construct spatial address
    UvoxId::new(
        RUm(r_um),
        lat_code,
        lon_code,
    )
}

// ---------------------------------------
/// Resolve a spatial address into a world-surface position.
/// ---------------------------------------

pub fn uvox_to_surface(
    world_id: WorldId,
    pos: UvoxId,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
) -> SurfaceCoords {
    // 1️⃣ Resolve world → cosmic body
    let anchor = world_state
        .anchors
        .get(&world_id)
        .expect("world has no anchor");

    let body_id = anchor.body;

    // 2️⃣ Get cosmic body reference radius
    let base_radius_m = cosmic_state
        .radii
        .get(&body_id)
        .expect("cosmic body has no radius")
        .meters
        .0;

    // 3️⃣ Decode angles (scaled integers → degrees)
    let latitude = code_to_degrees(pos.lat.0);
    let longitude = code_to_degrees(pos.lon.0);

    // 4️⃣ Decode elevation
    let r_m = pos.r_um.0 as f64 * 1e-6;
    let elevation = Meters(r_m - base_radius_m);

    SurfaceCoords {
        latitude,
        longitude,
        elevation,
    }
}


pub fn surface_coords_to_cosmic(
    world_id: WorldId,
    surface: SurfaceCoords,
    world_state: &WorldState,
    cosmic_state: &CosmicState,
) -> Vec3 {
    // Resolve world → body
    let anchor = world_state
        .anchors
        .get(&world_id)
        .expect("world has no anchor");

    let body_id = anchor.body;

    let base_radius =
        cosmic_state.radii.get(&body_id).unwrap().meters.0;

    let r = base_radius + surface.elevation.0;

    let lat = surface.latitude.0.to_radians();
    let lon = surface.longitude.0.to_radians();

    Vec3::new(
        r * lat.cos() * lon.cos(),
        r * lat.cos() * lon.sin(),
        r * lat.sin(),
    )
}
