//! Test Sun + Moon ephemeris using SimTime + UvoxId.
//!
//! Run:
//!     cargo run --bin scratch_ephemeris

use omnivox::core::tdt::sim_time::SimTime;
use omnivox::core::physox::astronomy::solar::*;
use omnivox::core::physox::astronomy::lunar::*;
use omnivox::core::uvoxid::UvoxId;
use chrono::TimeZone;

fn print_uvx(label: &str, id: UvoxId) {
    println!("{}:", label);
    println!("  r_um     = {}", id.r_um);
    println!("  lat_code = {}", id.lat_code);
    println!("  lon_code = {}", id.lon_code);
}

fn test_for(time: SimTime) {
    println!("-----------------------------------------------------------");
    println!("SimTime: {}", time.format_rfc3339());
    println!("Raw ns : {}", time.as_ns());
    println!("-----------------------------------------------------------");

    //
    // SUN
    //
    let (sun_ra, sun_dec, sun_dist) = solar_ra_dec(time);
    let sun_vec = solar_vector(time);
    let sun_uvx = solar_uvox(time);

    println!("SUN:");
    println!("  RA   = {:.6}°", sun_ra);
    println!("  Dec  = {:.6}°", sun_dec);
    println!("  Dist = {:.0} km", sun_dist);
    println!("  Vec  = [{:.0}, {:.0}, {:.0}] m",
        sun_vec.x, sun_vec.y, sun_vec.z
    );
    print_uvx("  UvoxId", sun_uvx);
    println!();

    //
    // MOON
    //
    let (moon_ra, moon_dec, moon_dist) = lunar_ra_dec(time);
    let moon_vec = lunar_vector(time);
    let moon_uvx = lunar_uvox(time);

    println!("MOON:");
    println!("  RA   = {:.6}°", moon_ra);
    println!("  Dec  = {:.6}°", moon_dec);
    println!("  Dist = {:.0} km", moon_dist);
    println!("  Vec  = [{:.0}, {:.0}, {:.0}] m",
        moon_vec.x, moon_vec.y, moon_vec.z
    );
    print_uvx("  UvoxId", moon_uvx);
    println!();
}

fn main() {
    println!("\n========== EPHEMERIS SCRATCH TEST ==========\n");

    //
    // 1. UNIX EPOCH
    //
    let t_unix = SimTime::from_ns(0);
    test_for(t_unix);

    //
    // 2. J2000 (should be near Meeus zero-point)
    //
    // J2000 = 2000-01-01T12:00:00 UTC
    let t_j2000 = SimTime::from_datetime(
        chrono::Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap()
    );
    test_for(t_j2000);

    //
    // 3. NOW (real-time check)
    //
    let t_now = SimTime::now();
    test_for(t_now);

    println!("\n========== END SCRATCH TEST ==========\n");
}
