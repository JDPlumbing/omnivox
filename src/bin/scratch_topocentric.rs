// src/bin/scratch_topocentric.rs
//! Test harness for topocentric solar computation.

use omnivox::core::tdt::sim_time::SimTime;
use omnivox::core::uvoxid::UvoxId;
use omnivox::core::physox::astronomy::topocentric::sun_topocentric;
use chrono::TimeZone;

/// Build a UvoxId on Earth's surface given lat/lon
fn uvox_from_latlon(lat_deg: f64, lon_deg: f64) -> UvoxId {
    const EARTH_R_UM: i64 = (6_371_000.0 * 1_000_000.0) as i64;

    UvoxId {
        r_um: EARTH_R_UM,
        lat_code: (lat_deg * 1e11).round() as i64,
        lon_code: (lon_deg * 1e11).round() as i64,
    }
}

fn main() {
    println!("========== TOPOCENTRIC SOLAR SCRATCH TEST ==========\n");

    // Miami, Florida
    let observer = uvox_from_latlon(25.7617, -80.1918);

    let tests = vec![
        (
            "1970-01-01T00:00:00Z",
            SimTime::from_datetime(
                chrono::Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
            ),
        ),
        (
            "2000-01-01T12:00:00Z",
            SimTime::from_datetime(
                chrono::Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap(),
            ),
        ),
        ("Now()", SimTime::now()),
    ];

    for (label, t) in tests {
        println!("-----------------------------------------------------------");
        println!("{label}: {}", t.to_datetime());
        println!("Raw ns : {}", t.as_ns());
        println!("-----------------------------------------------------------");

        let topo = sun_topocentric(observer, t);

        println!("Solar elevation : {:.6}°", topo.elevation_deg);
        println!("Solar azimuth   : {:.6}°", topo.azimuth_deg);
        println!("Zenith angle    : {:.6}°", topo.zenith_deg);
        println!("Hour angle      : {:.6}°", topo.hour_angle_deg);
        println!("Daylight        : {}", topo.is_daylight);
        println!("Irradiance fac. : {:.6}", topo.irradiance_factor);

        println!("Subsolar point:");
        println!("  lat_code = {}", topo.subsolar.lat_code);
        println!("  lon_code = {}", topo.subsolar.lon_code);
        println!();
    }

    println!("========== END SCRATCH TEST ==========");
}
