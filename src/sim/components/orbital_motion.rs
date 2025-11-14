use serde::{Serialize, Deserialize};

/// Orbital motion parameters governing how an object's UvoxId changes.
///
/// All values are per-second rates, allowing time-step independent motion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalMotion {
    /// Change in longitude per second (µdegrees per second)
    pub lon_rate_per_s: i64,

    /// Maximum latitude amplitude (µdegrees)
    pub lat_amp: i64,

    /// Direction of the tilt oscillation (+1 or -1)
    pub tilt_dir: i8,

    /// Change in latitude per second (derived every tick)
    pub lat_rate_per_s: i64,

    /// Mean orbital radius (µmeters)
    pub mean_r_um: i64,

    /// Radius variation amplitude (±µmeters)
    pub delta_r_um: i64,

    /// Direction of radial oscillation
    pub r_dir: i8,

    /// Rate of radial change per second (computed per tick)
    pub r_rate_per_s: i64,
}

impl OrbitalMotion {
    pub fn new_for_sun() -> Self {
        // Solar constants based on Earth-frame simplification
        // 360° per 31,557,600 seconds = ~1.141e-5 deg/s
        let lon_rate_per_s = (1.141e-5 * 1e6) as i64; // µdeg/s

        let lat_amp = 23_440_000_000_000; // ±23.44° * 1e11 µdeg
        let lat_rate_per_s =
            (2 * lat_amp) / (31_557_600 / 2); // half-year oscillation

        let mean_r_um = 1_496_000_000_000_000; // 1 AU
        let delta_r_um = 25_000_000_000_000;   // ± variation
        let r_rate_per_s =
            (2 * delta_r_um) / (31_557_600 / 2);

        Self {
            lon_rate_per_s,
            lat_amp,
            tilt_dir: 1,
            lat_rate_per_s,
            mean_r_um,
            delta_r_um,
            r_dir: 1,
            r_rate_per_s,
        }
    }
}
