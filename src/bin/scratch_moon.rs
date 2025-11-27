// src/bin/scratch_moon.rs

use omnivox::core::{
    tdt::sim_time::SimTime,
    physox::astronomy::{
        lunar::{lunar_ra_dec, lunar_vector, lunar_uvox},
    },
};
use chrono::TimeZone;

fn print_header(t: SimTime) {
    println!("-----------------------------------------------------------");
    println!("SimTime: {}", t.to_datetime());
    println!("Raw ns : {}", t.as_ns());
    println!("-----------------------------------------------------------");
}

fn main() {
    println!("========== MOON EPHEMERIS SCRATCH TEST ==========\n");

    // -------------------------------------------------------
    // Test 1: UNIX EPOCH (1970-01-01 00:00:00 UTC)
    // -------------------------------------------------------
    let t0 = SimTime::from_seconds(0);
    print_header(t0);

    let (ra, dec, dist_km) = lunar_ra_dec(t0);
    let vec = lunar_vector(t0);
    let uvox = lunar_uvox(t0);

    println!("MOON:");
    println!("  RA   = {:.6}°", ra);
    println!("  Dec  = {:.6}°", dec);
    println!("  Dist = {:.3} km", dist_km);
    println!("  Vec  = [{:.0}, {:.0}, {:.0}] m", vec.x, vec.y, vec.z);
    println!("  UvoxId:");
    println!("  r_um     = {}", uvox.r_um);
    println!("  lat_code = {}", uvox.lat_code);
    println!("  lon_code = {}", uvox.lon_code);
    println!();

    // -------------------------------------------------------
    // Test 2: J2000.0 (2000-01-01 12:00:00 UTC)
    // -------------------------------------------------------
    let t1 = SimTime::from_datetime(
        chrono::Utc.with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap()
    );
    print_header(t1);

    let (ra, dec, dist_km) = lunar_ra_dec(t1);
    let vec = lunar_vector(t1);
    let uvox = lunar_uvox(t1);

    println!("MOON:");
    println!("  RA   = {:.6}°", ra);
    println!("  Dec  = {:.6}°", dec);
    println!("  Dist = {:.3} km", dist_km);
    println!("  Vec  = [{:.0}, {:.0}, {:.0}] m", vec.x, vec.y, vec.z);
    println!("  UvoxId:");
    println!("  r_um     = {}", uvox.r_um);
    println!("  lat_code = {}", uvox.lat_code);
    println!("  lon_code = {}", uvox.lon_code);
    println!();

    // -------------------------------------------------------
    // Test 3: NOW
    // -------------------------------------------------------
    let t2 = SimTime::now();
    print_header(t2);

    let (ra, dec, dist_km) = lunar_ra_dec(t2);
    let vec = lunar_vector(t2);
    let uvox = lunar_uvox(t2);

    println!("MOON:");
    println!("  RA   = {:.6}°", ra);
    println!("  Dec  = {:.6}°", dec);
    println!("  Dist = {:.3} km", dist_km);
    println!("  Vec  = [{:.0}, {:.0}, {:.0}] m", vec.x, vec.y, vec.z);
    println!("  UvoxId:");
    println!("  r_um     = {}", uvox.r_um);
    println!("  lat_code = {}", uvox.lat_code);
    println!("  lon_code = {}", uvox.lon_code);
    println!();

    println!("========== END SCRATCH TEST ==========");
}
