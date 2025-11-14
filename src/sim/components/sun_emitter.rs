use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunEmitter {
    // Total power output of the Sun (W)
    pub luminosity_w: f64,

    // Angular motion (encoded in uvox lat/lon format)
    pub lon_step: i64,  // forward orbital sweep
    pub lat_amp: i64,   // seasonal oscillation amplitude
    pub tilt_dir: i8,   // +1 or -1 for tilt cycle direction
    pub orbit_dir: i8,  // +1 or -1 for orbital revolution direction

    // Fraction of solar output that is UV band (3–4% is realistic)
    pub uv_fraction: f64,
}

impl SunEmitter {
    pub fn new_for_sun() -> Self {
        Self {
            luminosity_w: 3.828e26,           // actual solar luminosity (watts)

            // Earth days converted to uvox lat/lon scaling
            lon_step: 1_000_000_000,          // about 1e9 µ-radians per timestep (you can tune)
            lat_amp: (23.44_f64 * 1e11) as i64, // 23.44° axial tilt in 1e11 scale

            tilt_dir: 1,
            orbit_dir: 1,

            uv_fraction: 0.037, // realistic UV power fraction ~3.7%
        }
    }
}
