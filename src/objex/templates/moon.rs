use crate::{
    objex::core::types::{Objex, MaterialLink, MaterialName},
    sim::world::WorldState,
    uvoxid::UvoxId,
};

/// Earth-centered, integer-driven Moon.
pub struct Moon {
    pub entity: Objex,

    // persistent physical state
    pub temperature_k: i64,
    pub albedo: i64,          // reflectivity scaled 0–10000 = 0.0–1.0
    pub lon_step: i64,        // µdeg per tick (orbital motion)
    pub lat_amp: i64,         // inclination ±5.14° in µdeg
    pub tilt_dir: i8,         // +1 or –1 for inclination oscillation
    pub orbit_dir: i8,        // +1 or –1 for perigee/apogee oscillation
}

impl Moon {
    pub fn new() -> Self {
        let mat = MaterialLink::new(MaterialName::Custom("Regolith".into()));
        let radius_m = 1.7374e6;
        let mut entity = Objex::new_sphere(0, None, mat, radius_m)
            .with_metadata("type", "satellite")
            .with_metadata("reflective", "true");
        entity.name = "Moon".to_string();

        // initial orbit: mean Earth–Moon distance
        entity.uvoxid = UvoxId::new(0, 384_400_000_000, 0, 0);

        // orbital period: 27.3217 days = 2,360,590 s
        let lon_step = (360.0 / 2_360_590.0 * 1e6) as i64; // µdeg per s
        let lat_amp = (5.145 * 1e11) as i64;               // ±5.145° in µdeg

        Self {
            entity,
            temperature_k: 220_000, // ~220 K average surface
            albedo: 1200,           // 0.12 reflectivity scaled *1e4
            lon_step,
            lat_amp,
            tilt_dir: 1,
            orbit_dir: 1,
        }
    }

    /// Advance apparent position using integer math.
    pub fn tick(&mut self, _world: &mut WorldState, _dt_s: i64) {
        const LON_WRAP: i64 = 360 * 1_000_000;
        const MAX_TILT: i64 = 5_145_000_000_000; // ±5.145° in 1e11 µdeg
        const ORBIT_STEPS: i64 = 2_360_590;      // seconds per orbit
        const MEAN_R_UM: i64 = 384_400_000_000;
        const DELTA_R_UM: i64 = 21_000_000_000;  // ±21e9 µm (~21 km)

        // longitude
        self.entity.uvoxid.lon_code =
            (self.entity.uvoxid.lon_code + self.lon_step) % LON_WRAP;

        // latitude oscillation (inclination)
        let lat_step = (2 * MAX_TILT) / (ORBIT_STEPS / 2);
        self.entity.uvoxid.lat_code += lat_step * self.tilt_dir as i64;
        if self.entity.uvoxid.lat_code.abs() >= MAX_TILT {
            self.tilt_dir *= -1;
        }

        // radius oscillation (perigee/apogee)
        let step_r = (2 * DELTA_R_UM) / (ORBIT_STEPS / 2);
        self.entity.uvoxid.r_um += step_r * self.orbit_dir as i64;
        if self.entity.uvoxid.r_um >= MEAN_R_UM + DELTA_R_UM
            || self.entity.uvoxid.r_um <= MEAN_R_UM - DELTA_R_UM
        {
            self.orbit_dir *= -1;
        }
    }
}
