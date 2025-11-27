//! Solar ephemeris scratch test
//! Place in: src/bin/scratch_sun.rs
//! Run with: cargo run --bin scratch_sun

use omnivox::core::tdt::sim_time::SimTime;
use omnivox::core::physox::astronomy::solar::solar_position;
use chrono::TimeZone;

fn print_sun(t: SimTime) {
    println!("-----------------------------------------------------------");
    println!("SimTime: {}", t.to_datetime().to_rfc3339());
    println!("Raw ns : {}", t.0);

    let sun = solar_position(t);

    println!("-----------------------------------------------------------");
    println!("SUN:");
    println!("  RA   = {:.6}°", sun.ra_deg);
    println!("  Dec  = {:.6}°", sun.dec_deg);
    println!("  Dist = {:.3} km", sun.dist_km);

    println!(
        "  Vec  = [{:.0}, {:.0}, {:.0}] m",
        sun.vec_m.x, sun.vec_m.y, sun.vec_m.z
    );

    println!("  UvoxId:");
    println!("  r_um     = {}", sun.uvox.r_um);
    println!("  lat_code = {}", sun.uvox.lat_code);
    println!("  lon_code = {}", sun.uvox.lon_code);
}

fn main() {
    println!("========== EPHEMERIS SCRATCH TEST ==========\n");

    // 1. Unix epoch (1970-01-01 00:00:00 UTC)
    let t1 = SimTime::from_datetime(chrono::Utc
        .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
        .unwrap());
    print_sun(t1);

    // 2. J2000 epoch (2000-01-01 12:00:00 UTC)
    let t2 = SimTime::from_datetime(chrono::Utc
        .with_ymd_and_hms(2000, 1, 1, 12, 0, 0)
        .unwrap());
    print_sun(t2);

    // 3. Right now
    let t3 = SimTime::now();
    print_sun(t3);

    println!("\n========== END SCRATCH TEST ==========");
}
