use crate::{
    objex::core::types::{Objex, MaterialLink, MaterialName},
    sim::world::WorldState,
    uvoxid::UvoxId,
};

/// Sun-centered, integer-driven Earth.
pub struct Earth {
    pub entity: Objex,

    // persistent physical state
    pub temperature_k: i64,
    pub albedo: i64,          // reflectivity scaled 0–10000 = 0.0–1.0
    pub lon_step: i64,        // µdeg per tick (orbital motion)
    pub lat_amp: i64,         // axial tilt amplitude scaled like before
    pub tilt_dir: i8,         // +1 or –1 for inclination/axial oscillation
    pub orbit_dir: i8,        // +1 or –1 for perihelion/aphelion oscillation
}

impl Earth {
    pub fn new() -> Self {
        let mat = MaterialLink::new(MaterialName::Custom("Earth".into()));
        let radius_m = 6.371e6;
        let mut entity = Objex::new_sphere(0, None, mat, radius_m)
            .with_metadata("type", "planet")
            .with_metadata("habitable", "true");
        entity.name = "Earth".to_string();

        // initial orbit: mean Sun–Earth distance (approx 1 AU)
        entity.uvoxid = UvoxId::new(0, 149_600_000_000_000, 0, 0);

        // orbital period: ~365.25 days ≈ 31,557,600 s
        let lon_step = (360.0 / 31_557_600.0 * 1e6) as i64; // µdeg per s
        let lat_amp = (23.44 * 1e11) as i64;                // ≈23.44° in same scaling

        Self {
            entity,
            temperature_k: 288_000, // ~288 K average surface (scaled like original)
            albedo: 3060,           // ~0.306 reflectivity scaled *1e4
            lon_step,
            lat_amp,
            tilt_dir: 1,
            orbit_dir: 1,
        }
    }

    /// Advance apparent position using integer math.
    pub fn tick(&mut self, _world: &mut WorldState, _dt_s: i64) {
        const LON_WRAP: i64 = 360 * 1_000_000;
        const MAX_TILT: i64 = 23_440_000_000_000; // ±23.44° in same scaling as lat_amp
        const ORBIT_STEPS: i64 = 31_557_600;      // seconds per orbit (~1 year)
        const MEAN_R_UM: i64 = 149_600_000_000_000; // mean Sun–Earth distance in same units as uvoxid.r_um
        const DELTA_R_UM: i64 = 2_500_000_000_000;  // ±~2.5e12 (approx perihelion/aphelion range)

        // longitude
        self.entity.uvoxid.lon_code =
            (self.entity.uvoxid.lon_code + self.lon_step) % LON_WRAP;

        // latitude/axial-tilt oscillation
        let lat_step = (2 * MAX_TILT) / (ORBIT_STEPS / 2);
        self.entity.uvoxid.lat_code += lat_step * self.tilt_dir as i64;
        if self.entity.uvoxid.lat_code.abs() >= MAX_TILT {
            self.tilt_dir *= -1;
        }

        // radius oscillation (perihelion/aphelion)
        let step_r = (2 * DELTA_R_UM) / (ORBIT_STEPS / 2);
        self.entity.uvoxid.r_um += step_r * self.orbit_dir as i64;
        if self.entity.uvoxid.r_um >= MEAN_R_UM + DELTA_R_UM
            || self.entity.uvoxid.r_um <= MEAN_R_UM - DELTA_R_UM
        {
            self.orbit_dir *= -1;
        }
    }
}
